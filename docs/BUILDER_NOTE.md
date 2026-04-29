# Builder Note: Verifiable Trading Journal Proof on Nexus

I am experimenting with Nexus zkVM for a realistic trader workflow: proving that a trading journal summary was computed correctly without revealing every raw trade.

## Problem

Discretionary traders often share performance summaries: net R, win rate, max drawdown, risk used, and fees. These numbers are easy to manipulate and hard to verify without exposing private trade data.

## Small proof-of-concept

This repo proves a tiny version of the problem:

- Private input: encoded trade results.
- Public input: expected journal score.
- Public output: verified score.

The guest program computes net R in half-R units. The host program proves and verifies the execution with Nexus zkVM.

## Why Nexus

Nexus is a natural fit because the core need is not trading signals. It is verifiable computation: prove that a calculation was executed correctly.

## Next steps

- Expand from net-R proof to max drawdown proof.
- Add a helper that converts exported journal CSV/JSON into zkVM input.
- Integrate with a future Decision Journal export format.
