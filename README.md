# Helix Shell Plugins

Some simple [helix](https://helix-editor.com/) shell extension plugins I use.
Install with `cargo install` or however you want! Just make sure the binary is available to your helix process.

## Config

From normal mode, I use `<space><space>` to enter my plugins menu:

```toml
[keys.normal.space.space]
d = ":pipe details"
```

## Plugins

### Details

Makes fancy hidden markdown blocks!

<details>
  <summary>Turn this</summary>

  ```
  Summary Line
  Hidden Text
  Hidden Text
  ```

</details>

<details>
  <summary>Into this</summary>

  ```md
  <details>
    <summary>Summary Line</summary>
  
    Hidden Text
    Hidden Text
  
  </details>
  ```

</details>

The same command will also undo the markup so you can easily edit again.
