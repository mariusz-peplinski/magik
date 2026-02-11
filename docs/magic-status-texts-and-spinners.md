# Magic UI Status Texts And Spinners

This repo now supports customizing spinner status text in the TUI via JSON.

The file is loaded from:

- `~/.magic/status_texts.json`

## What it changes

When the top border spinner status would normally show labels like `Thinking`, `Responding`, or `Using tools`, Magic can pick a random phrase from your configured list.

It picks a new random phrase each time the status updates.

## JSON format

```json
{
  "groups": {
    "all": ["figuring things out", "trying random stuff", "thinking hard"],
    "thinking_set": ["thinking hard", "connecting dots", "cooking ideas"],
    "response_set": ["wordsmithing", "drafting reply", "polishing answer"]
  },
  "status_groups": {
    "thinking": "thinking_set",
    "responding": "response_set",
    "tools": "thinking_set",
    "all": "all"
  }
}
```

## Status keys

Use these keys in `status_groups`:

- `auto_review`
- `auto_drive_goal`
- `auto_drive`
- `thinking`
- `tools`
- `browsing`
- `agents`
- `responding`
- `reconnecting`
- `coding`
- `reading`
- `working`

## Group routing behavior

- If a status has an explicit mapping in `status_groups`, that group is used.
- Otherwise, if `status_groups.all` exists, that group is used.
- Otherwise, if `groups.all` exists, that group is used.
- Otherwise, the default built-in status label is used.

## Shared groups across statuses

To make multiple statuses use the same text list, map them to the same group name:

```json
{
  "groups": {
    "my_shared": ["in the zone", "deep focus", "making moves"]
  },
  "status_groups": {
    "thinking": "my_shared",
    "responding": "my_shared",
    "tools": "my_shared"
  }
}
```

## New GPT spinner set

Added built-in spinner options under the `GPT` group:

- `gptPulseCore`
- `gptNeonFlip`
- `gptWaveTrail`
- `gptOrbit`
- `gptPixelRain`
- `gptSignal`
- `gptComet`
- `gptHeartbeat`

You can select any spinner using your existing TUI spinner config.

