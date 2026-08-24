// Total, bounded alarm occurrence model. Unknown/invalid pairs stutter;
// stale generations cannot mutate the current occurrence.
module alarm_occurrence {
  type State = {
    phase: int,
    generation: int,
    snooze_set: bool,
    acknowledged: bool,
    terminal: bool,
  }

  pure val SCHEDULED = 0
  pure val FIRING = 1
  pure val ACKNOWLEDGED = 2
  pure val SNOOZED = 3
  pure val COMPLETED = 4
  pure val MISSED = 5
  pure val CANCELED = 6

  pure val FIRE = 0
  pure val ACKNOWLEDGE = 1
  pure val SNOOZE = 2
  pure val COMPLETE = 3
  pure val MARK_MISSED = 4
  pure val CANCEL = 5

  pure val MAX_GENERATION = 5
  pure val PHASES = Set(SCHEDULED, FIRING, ACKNOWLEDGED, SNOOZED, COMPLETED, MISSED, CANCELED)
  pure val EVENTS = Set(FIRE, ACKNOWLEDGE, SNOOZE, COMPLETE, MARK_MISSED, CANCEL)
  pure val TOKENS = Set(0, 1, 2, 3, 4, 5, 99)

  var s: State

  pure def is_terminal(phase: int): bool = Set(COMPLETED, MISSED, CANCELED).contains(phase)

  pure def next_phase(current: int, event: int, has_snooze: bool): int =
    if (current == SCHEDULED and event == FIRE) FIRING
    else if (current == SCHEDULED and event == CANCEL) CANCELED
    else if (current == SCHEDULED and event == MARK_MISSED) MISSED
    else if (current == FIRING and event == ACKNOWLEDGE) ACKNOWLEDGED
    else if (current == FIRING and event == SNOOZE and has_snooze) SNOOZED
    else if (current == FIRING and event == MARK_MISSED) MISSED
    else if (current == FIRING and event == CANCEL) CANCELED
    else if (current == ACKNOWLEDGED and event == COMPLETE) COMPLETED
    else if (current == ACKNOWLEDGED and event == SNOOZE and has_snooze) SNOOZED
    else if (current == ACKNOWLEDGED and event == CANCEL) CANCELED
    else if (current == SNOOZED and event == FIRE) FIRING
    else if (current == SNOOZED and event == CANCEL) CANCELED
    else current

  pure def transition(current: State, event: int, expected: int, has_snooze: bool): State = {
    val candidate = next_phase(current.phase, event, has_snooze)
    if (
      expected != current.generation
      or current.generation >= MAX_GENERATION
      or is_terminal(current.phase)
      or candidate == current.phase
    ) current
    else {
      phase: candidate,
      generation: current.generation + 1,
      snooze_set: candidate == SNOOZED,
      acknowledged: current.acknowledged or candidate == ACKNOWLEDGED or candidate == COMPLETED,
      terminal: is_terminal(candidate),
    }
  }

  action init = all {
    s' = { phase: SCHEDULED, generation: 0, snooze_set: false, acknowledged: false, terminal: false },
  }

  action step = nondet event = EVENTS.oneOf() {
    nondet expected = TOKENS.oneOf() {
      nondet has_snooze = Set(true, false).oneOf() {
        s' = transition(s, event, expected, has_snooze)
      }
    }
  }

  val occurrence_safety = and {
    PHASES.contains(s.phase),
    s.generation >= 0,
    s.generation <= MAX_GENERATION,
    s.snooze_set == (s.phase == SNOOZED),
    s.terminal == is_terminal(s.phase),
    s.phase == COMPLETED implies s.acknowledged,
  }
}

