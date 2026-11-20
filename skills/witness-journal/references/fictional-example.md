# Fictional example

Every detail and ID below is invented. In this example the owner asks for a letter about the community lantern workshop on August 14, 2026, using their workshop note. The system date is August 15, 2026.

## Authorized evidence

- **S1** is workshop note 705bee73-2830-491b-bbe6-18868f84021e, revision 1, attributed to the owner. It records that the owner said they stayed until the last lantern was lit. The note is a paraphrase. It contains no exact quote and does not independently establish how the owner felt.

## Private draft

### A letter after the workshop

- **Entry date** August 15, 2026
- **Period covered** August 14, 2026
- **Writer** Finch, the witnessing agent

You told me you stayed until the last lantern was lit. Your workshop note records that detail and gives me a small moment to hold on to. [S1]

I read your staying as a quiet way of seeing the evening through. I don’t know what the moment felt like to you. It could have mattered in ways the note does not show. [S1]

**S1** Owner’s workshop note, record 705bee73-2830-491b-bbe6-18868f84021e, revision 1.

The draft remains in the conversation. The owner can leave it there, ask for edits, or say “Submit this draft for my review.”

## Submit the approved draft to the inbox

After the owner asks, send a request like this to mother_propose_memory. In an actual run, read the current workspace and generation and generate a fresh operation UUID.

```json
{
  "workspaceId": "d3bca0a0-7f52-4b4f-a64c-38baffe85681",
  "generation": "59feba05-c87a-48f9-8171-fb08c6234ec4",
  "operationId": "66cb3c49-92ef-4c3c-9592-c2313e056636",
  "expectedRevision": 0,
  "source": "Finch, witnessing agent. August 15, 2026 letter about August 14, 2026. Owner's workshop note, record 705bee73-2830-491b-bbe6-18868f84021e revision 1.",
  "basis": "model_inference",
  "record": {
    "kind": "memory",
    "title": "Witness Journal | August 14, 2026 | A letter after the workshop",
    "content": "Entry date: August 15, 2026\nPeriod covered: August 14, 2026\nWriter: Finch, the witnessing agent\n\nYou told me you stayed until the last lantern was lit. Your workshop note records that detail and gives me a small moment to hold on to. [S1]\n\nI read your staying as a quiet way of seeing the evening through. I don’t know what the moment felt like to you. It could have mattered in ways the note does not show. [S1]\n\nS1: Owner’s workshop note, record 705bee73-2830-491b-bbe6-18868f84021e, revision 1.",
    "status": "active",
    "basis": "model_inference",
    "references": ["705bee73-2830-491b-bbe6-18868f84021e"]
  }
}
```

The harness reports that the proposal is awaiting owner review. In this fictional run the owner approves it as record 6c377ea8-dc5b-41f3-bfe4-63ab583488ee, revision 1.

## Correct the letter

On August 17, the owner explains that they left before the last lantern was lit. The correction is recorded in a separate owner note **S2**, record 139d6ffc-bef8-444f-a08e-51e5877f1e08, revision 1. A second witnessing agent named Reed reads the existing letter and both notes in full and drafts this replacement.

> - **Entry date** August 15, 2026
> - **Period covered** August 14, 2026
> - **Original writer** Finch
> - **Correction dated** August 17, 2026, prepared by Reed
>
> You later explained that you left before the last lantern was lit. [S2] In the original letter Finch read your reported staying as a way of seeing the evening through. [S1] That reading rested on a detail the later note corrects. I’ll leave the reason you left open. Neither note establishes it.
>
> - **S1** Original owner workshop note, record 705bee73-2830-491b-bbe6-18868f84021e, revision 1. Retained as the original letter’s evidence.
> - **S2** Owner correction note, record 139d6ffc-bef8-444f-a08e-51e5877f1e08, revision 1.
> - **Earlier letter** Record 6c377ea8-dc5b-41f3-bfe4-63ab583488ee, revision 1, written by Finch.

The owner asks Reed to submit this correction. Reed sends a new proposal with the same record ID and its current revision. The source field credits both writers.

```json
{
  "workspaceId": "d3bca0a0-7f52-4b4f-a64c-38baffe85681",
  "generation": "59feba05-c87a-48f9-8171-fb08c6234ec4",
  "operationId": "03e90d92-0181-4492-b953-c62c394c8222",
  "id": "6c377ea8-dc5b-41f3-bfe4-63ab583488ee",
  "expectedRevision": 1,
  "source": "August 17, 2026 correction by Reed to Finch's original letter, record 6c377ea8-dc5b-41f3-bfe4-63ab583488ee revision 1, based on the original workshop note and the owner's correction note.",
  "basis": "model_inference",
  "record": {
    "kind": "memory",
    "title": "Witness Journal | August 14, 2026 | A letter after the workshop",
    "content": "Entry date: August 15, 2026\nPeriod covered: August 14, 2026\nOriginal writer: Finch\nCorrection dated: August 17, 2026, prepared by Reed\n\nYou later explained that you left before the last lantern was lit. [S2] In the original letter Finch read your reported staying as a way of seeing the evening through. [S1] That reading rested on a detail the later note corrects. I’ll leave the reason you left open. Neither note establishes it.\n\nS1: Original owner workshop note, record 705bee73-2830-491b-bbe6-18868f84021e, revision 1. Retained as the original letter’s evidence.\nS2: Owner correction note, record 139d6ffc-bef8-444f-a08e-51e5877f1e08, revision 1.\nEarlier letter: Record 6c377ea8-dc5b-41f3-bfe4-63ab583488ee, revision 1, written by Finch.",
    "status": "active",
    "basis": "model_inference",
    "references": [
      "705bee73-2830-491b-bbe6-18868f84021e",
      "139d6ffc-bef8-444f-a08e-51e5877f1e08"
    ]
  }
}
```

The proposal stays pending until the owner approves. Approval yields revision 2 under the same ID. An authorized history read with that ID and revision 1 still returns Finch's original wording and citations.
