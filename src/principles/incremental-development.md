# Incremental Development

Software is never finished. It evolves. The only question is whether it evolves in a controlled, understandable way — or in a chaotic, frightening one.

Incremental development is the practice of making software in small, safe, verifiable steps. TDD enforces this, but the principle goes deeper than the test cycle.

## Small steps

Each step should:
- Leave the code in a working state (tests green)
- Add one piece of meaningful behaviour
- Be small enough that you could throw it away without losing much

When a step feels big, break it down. If you can't break it down, that's often a signal the design is more tangled than it should be.

## The "walking skeleton"

When starting something new, the first goal is not to build a feature — it's to build a thin, end-to-end slice that actually runs. A skeleton. Then you flesh it out.

This is more important than it sounds. Getting everything connected early — even in a trivial, incomplete way — surfaces integration problems before you've invested too much in each layer.

## Commit often

Every time your tests are green and you've made meaningful progress, consider committing. Source control is not just a backup — it's a way to document your thinking and give yourself a safe point to return to if a refactor goes wrong.

LGWT's Hello World chapter explicitly says: once tests pass, commit before you refactor. That discipline is worth adopting.

## The ratchet

Think of incremental development as a ratchet: you can only turn it one way. Each green test locks in a behaviour. You can refactor around it, but you can't accidentally un-implement it without noticing.

This is what makes TDD feel safe. The test suite is a ratchet on correctness.

## Resist the urge to over-engineer

Incremental development means trusting that you can add complexity later *when you need it*. Don't add it speculatively.

> "You ain't gonna need it." — YAGNI

The flip side: don't build so incrementally that you never integrate. Small steps towards a real goal, not small steps that circle endlessly.

## See also

- [The TDD Cycle](./tdd-cycle.md)
- [Outside-In TDD](./outside-in.md)
