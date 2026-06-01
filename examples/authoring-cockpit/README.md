# Authoring Cockpit v0

Minimal browser UI for inspecting the existing game-runtime scene and editing only the fields supported by the Rust scene edit model.

Run locally from the repo root:

```bash
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Open <http://127.0.0.1:8000/examples/authoring-cockpit/>.

The inspector updates scene data in memory and shows the validated `ouroforge scene edit` command for writing through Rust-side validation. Direct browser file writes are intentionally not supported.
