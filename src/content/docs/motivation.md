---
title: Motivation & Requirements
description: Why the Suffragio project designed a modern e-voting system, and the goals and requirements behind it.
---

Suffragio is a proposal for a modern electoral system. This page describes the motivation behind it, together with the goals and requirements that guided its design.

## Requirements

### Low-cost election administration

A member of a national election commission (or of any organization) should be able to design, publish, and verify an election and its results without significant cost.

### Universality

The system must be able to support any kind of election — national elections in particular, but not exclusively. This includes various question types (yes/no, single choice, multiple choice, 1–10 scale, etc.) as well as non-trivial ballots (referendums, party lists, votes for individual candidates, and different ballot content for different constituencies within the same election).

### Support for any electoral formula

The legal system of a given country, or the type of election being held, should never be a reason to build yet another separate system. The results — who becomes president, or who is seated in parliament — should be computable from the collected votes according to whichever formula applies.

### Verifiability and openness

Every citizen should be able to independently verify the election result locally. This requires full access to all cast ballots, as well as access to the source code of every component of the electoral system.

### One ballot per eligible voter

Every person entitled to vote may download a ballot only once, and only for their own constituency.

### Ballot secrecy

No one — in particular, the government — should be able to determine how any individual voted.

### Identity verification

Election organizers must have access to identity- and eligibility-verification tools. It must be possible to verify identity both electronically (e.g. using a qualified electronic signature or a national digital identity service) and in person, for example by an official at a municipal, county, or embassy/consular office.

### Revocation of voting rights

If a person has lost their voting rights (e.g. due to a legally binding conviction that carries disenfranchisement, intellectual disability, or death), the system must refuse to issue a ballot to them.

### Authentication & authorization

The system must support integration with multiple types of identity credentials, along with a fine-grained permission model that allows delegation of authority between institutions — for example, granting or revoking the right to: verify identity in person, organize elections, or edit the set of questions on a ballot.

### Discovery & broadcast

There must be a mechanism for announcing and discovering available elections and network nodes.

## Full auditability

### Software integrity

Every citizen verifying an election result must be able to be certain that the software they are running is exactly the same software the government used during the election, and that it was not modified after the election began.

### Integrity and open ballot boxes

Every citizen has access to all ballots. Once cast, a vote can never be modified or deleted.

### Public electoral rolls

The list of people eligible to vote, and their assignment to constituencies, must be publicly available. It is an open question whether information about *who has downloaded a ballot* (as opposed to how they voted) should also be public.

### Digital independence

The software must be built on open protocols and provide complete API documentation. The specification must be complete enough that independent organizations can build separate, mutually compatible implementations without access to each other's source code.

The reference implementation must be released under a free, copyleft license — one that guarantees the freedom to use, modify, and distribute the software while requiring that these same freedoms be preserved for all downstream recipients.

The specification must not require the use of any specific closed standard, any specific vendor's product, or access to any service that is not held to the same or higher openness requirements as the e-voting system itself. The software must not be tied to any particular infrastructure, and everyone must be able to run it on their own machine. The reference implementation must include complete build instructions.

## Optional goals

- **Prevention of vote selling.**
- **Prevention of coercion** — protecting voters from being forced, under threat of violence, to cast a specific vote.
