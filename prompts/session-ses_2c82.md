# Cargo.toml metadata for Dart game score calculator

**Session ID:** ses_2c82a417dffeObsMMpF2mUP04C
**Created:** 3/28/2026, 10:24:08 PM
**Updated:** 3/28/2026, 10:36:59 PM

---

## User

Add Cargo.toml metadata:

description = "Dart game score calculator"
license = "MIT"
repository = "https://github.com/FedericoAguirre/darts/" 

---

## Assistant (Build · mimo-v2-omni-free · 1.9s)

_Thinking:_

I need to add metadata to the Cargo.toml file. First, let me check if there's already a Cargo.toml file in the project. Let's explore the directory structure to understand the codebase.

**Tool: read**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts"
}
```

**Output:**
```
<path>/home/federico/Documents/projects/darts</path>
<type>directory</type>
<entries>
.git/
.gitignore
Cargo.lock
Cargo.toml
README.md
src/
target/
tests/

(8 entries)
</entries>
```

---

## Assistant (Build · mimo-v2-omni-free · 1.5s)

_Thinking:_

Great, there's already a Cargo.toml file. Let's read it to see its current content and then add the metadata.

**Tool: read**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml"
}
```

**Output:**
```
<path>/home/federico/Documents/projects/darts/Cargo.toml</path>
<type>file</type>
<content>1: [package]
2: name = "darts"
3: version = "0.1.0"
4: edition = "2021"
5: 
6: # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
7: 
8: [dependencies]
9: clap = "4.4.11"
10: 
11: [lib]
12: name = "darts_lib"
13: path = "src/lib.rs"
14: 
15: [[bin]]
16: name = "darts"
17: path = "src/main.rs"

(End of file - total 17 lines)
</content>
```

---

## Assistant (Build · mimo-v2-omni-free · 5.0s)

_Thinking:_

Now I need to add description, license, and repository fields to the [package] section. According to Cargo.toml syntax, these fields should be added under [package]. Let's add them after the edition line (or before the comment). I'll add them after line 4 (edition) and before the comment line. Let's do an edit.

**Tool: edit**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml",
  "oldString": "[package]\nname = \"darts\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html",
  "newString": "[package]\nname = \"darts\"\nversion = \"0.1.0\"\nedition = \"2021\"\ndescription = \"Dart game score calculator\"\nlicense = \"MIT\"\nrepository = \"https://github.com/FedericoAguirre/darts/\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html"
}
```

**Output:**
```
Edit applied successfully.
```

---

## Assistant (Build · mimo-v2-omni-free · 2.3s)

_Thinking:_

Now let's verify the change by reading the file again.

**Tool: read**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml"
}
```

**Output:**
```
<path>/home/federico/Documents/projects/darts/Cargo.toml</path>
<type>file</type>
<content>1: [package]
2: name = "darts"
3: version = "0.1.0"
4: edition = "2021"
5: description = "Dart game score calculator"
6: license = "MIT"
7: repository = "https://github.com/FedericoAguirre/darts/"
8: 
9: # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
10: 
11: [dependencies]
12: clap = "4.4.11"
13: 
14: [lib]
15: name = "darts_lib"
16: path = "src/lib.rs"
17: 
18: [[bin]]
19: name = "darts"
20: path = "src/main.rs"

(End of file - total 20 lines)
</content>
```

---

## Assistant (Build · mimo-v2-omni-free · 5.8s)

_Thinking:_

Perfect. The metadata has been added successfully. Now I should also check if there's any need to run cargo check or other validation to ensure the metadata is valid. Let's run a quick cargo check to see if the manifest is valid.

**Tool: bash**

**Input:**
```json
{
  "command": "cargo check",
  "description": "Check Cargo.toml validity"
}
```

**Output:**
```
    Checking darts v0.1.0 (/home/federico/Documents/projects/darts)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.47s

```

---

## Assistant (Build · mimo-v2-omni-free · 1.7s)

_Thinking:_

Great. No errors. The metadata has been added successfully. The task is complete. Now I'll provide a concise response to the user.

Added the requested metadata to `Cargo.toml:5-7`. The package now includes description, license, and repository fields.

---

## User

Modify the release rust compilation so I can get the smallest and fastest binary

---

## Assistant (Build · mimo-v2-omni-free · 13.0s)

_Thinking:_

I need to modify the Cargo.toml to optimize for release builds. In Rust, you can add a profile for release with specific optimizations. The user wants the smallest and fastest binary. This typically involves:

1. Using `opt-level = "z"` or `"s"` for size optimization, or `"3"` for speed
2. Using `lto = true` for link-time optimization
3. Using `codegen-units = 1` for better optimization
4. Using `strip = true` to remove debug symbols
5. Using `panic = "abort"` to reduce binary size

Since the user wants both smallest and fastest, I'll use `opt-level = "z"` (optimize for size, but "s" is also for size, "z" is more aggressive) or maybe `opt-level = 3` for speed, but they want smallest and fastest, which can be conflicting. Usually `opt-level = "z"` gives smallest, `opt-level = 3` gives fastest. However, for binary size, we can also use other flags.

Let me check if there's already a profile section. Let me read the current Cargo.toml again and see if there's a profile.release section. If not, I'll add one.

First, let me examine the current Cargo.toml content again. I already read it, but I'll check for any profile section. The current content doesn't have a profile section. I'll add it at the end of the file.

I'll add a [profile.release] section with the typical flags for smallest and fastest binary. I'll use:
- `opt-level = "z"` for size optimization (more aggressive than "s")
- `lto = true` for link-time optimization
- `codegen-units = 1` for better optimization
- `strip = true` to strip debug info (requires Rust 1.59+)
- `panic = "abort"` to unwind tables (reduces size)

Let me write the edit. First, I need to ensure I know where to add it. I'll add after the [[bin]] section.

I'll read the file again to get the exact last lines.

**Tool: read**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml",
  "offset": 15
}
```

**Output:**
```
<path>/home/federico/Documents/projects/darts/Cargo.toml</path>
<type>file</type>
<content>15: name = "darts_lib"
16: path = "src/lib.rs"
17: 
18: [[bin]]
19: name = "darts"
20: path = "src/main.rs"

(End of file - total 20 lines)
</content>
```

---

## Assistant (Build · mimo-v2-omni-free · 3.2s)

_Thinking:_

I'll add the profile.release section after the [[bin]] section. I'll also add a comment about what each setting does. Let me do the edit.

**Tool: edit**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml",
  "oldString": "[[bin]]\nname = \"darts\"\npath = \"src/main.rs\"",
  "newString": "[[bin]]\nname = \"darts\"\npath = \"src/main.rs\"\n\n[profile.release]\nopt-level = \"z\"  # Optimize for size\nlto = true       # Enable link-time optimization\ncodegen-units = 1  # Single codegen unit for better optimization\nstrip = true     # Strip debug symbols\npanic = \"abort\"  # Abort on panic for smaller binary"
}
```

**Output:**
```
Edit applied successfully.
```

---

## Assistant (Build · mimo-v2-omni-free · 5.0s)

_Thinking:_

Now I should verify the changes and also run cargo check to ensure the configuration is valid. Let's read the file again to confirm.

**Tool: read**

**Input:**
```json
{
  "filePath": "/home/federico/Documents/projects/darts/Cargo.toml"
}
```

**Output:**
```
<path>/home/federico/Documents/projects/darts/Cargo.toml</path>
<type>file</type>
<content>1: [package]
2: name = "darts"
3: version = "0.1.0"
4: edition = "2021"
5: description = "Dart game score calculator"
6: license = "MIT"
7: repository = "https://github.com/FedericoAguirre/darts/"
8: 
9: # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
10: 
11: [dependencies]
12: clap = "4.4.11"
13: 
14: [lib]
15: name = "darts_lib"
16: path = "src/lib.rs"
17: 
18: [[bin]]
19: name = "darts"
20: path = "src/main.rs"
21: 
22: [profile.release]
23: opt-level = "z"  # Optimize for size
24: lto = true       # Enable link-time optimization
25: codegen-units = 1  # Single codegen unit for better optimization
26: strip = true     # Strip debug symbols
27: panic = "abort"  # Abort on panic for smaller binary

(End of file - total 27 lines)
</content>
```

---

## Assistant (Build · mimo-v2-omni-free · 2.3s)

_Thinking:_

Now let's test that cargo check still works with the new profile. Also, we could try building a release binary to see if it works, but that might take time. I'll just run cargo check to ensure the manifest is valid. Let's also check if there's a .cargo directory with config, but we can ignore that. Let's run cargo check again.

**Tool: bash**

**Input:**
```json
{
  "command": "cargo check",
  "description": "Verify Cargo.toml changes"
}
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s

```

---

## Assistant (Build · mimo-v2-omni-free · 12.9s)

_Thinking:_

Great. Now we can also run `cargo build --release` to see if it builds successfully. However, the user might want to know the commands to build the release binary. I can provide a quick instruction. But first, let's ensure it builds. We'll run a quick release build. This might take a moment. Let's do it.

**Tool: bash**

**Input:**
```json
{
  "command": "cargo build --release 2>&1 | tail -20",
  "description": "Build release binary to verify"
}
```

**Output:**
```
   Compiling utf8parse v0.2.1
   Compiling anstyle v1.0.4
   Compiling anstyle-query v1.0.2
   Compiling colorchoice v1.0.0
   Compiling clap_lex v0.6.0
   Compiling strsim v0.10.0
   Compiling anstyle-parse v0.2.3
   Compiling anstream v0.6.5
   Compiling clap_builder v4.4.11
   Compiling clap v4.4.11
   Compiling darts v0.1.0 (/home/federico/Documents/projects/darts)
    Finished `release` profile [optimized] target(s) in 10.69s

```

---

## Assistant (Build · mimo-v2-omni-free · 12.2s)

_Thinking:_

Excellent. The build succeeded. Now we can also check the size of the resulting binary. Let's locate the binary and check its size. Usually it's in `target/release/darts`. Let's check.

**Tool: bash**

**Input:**
```json
{
  "command": "ls -lh target/release/darts 2>/dev/null || echo \"Binary not found\"",
  "description": "Check binary size"
}
```

**Output:**
```
-rwxrwxr-x 2 federico federico 538K Mar 28 22:27 target/release/darts

```

---

## Assistant (Build · mimo-v2-omni-free · 4.7s)

_Thinking:_

Great, 538K. That's fairly small for a Rust binary with clap. However, we might be able to reduce further by using `opt-level = "s"` instead of "z". "z" is more aggressive for size than "s". Actually "z" is the most aggressive size optimization. It's fine.

Now, we could also consider adding `rustflags = ["-C", "target-cpu=native"]` for speed, but that would make the binary not portable. Since the user wants both smallest and fastest, we can leave as is. The current configuration is good.

Let's also consider adding `[profile.release.build-override]` but not necessary.

We should also note that the user may want to use `cargo strip` or `strip` command after building, but `strip = true` does that.

Thus the task is done. I'll provide a short summary.

Added a `[profile.release]` section to `Cargo.toml` with optimizations for small, fast binaries:

- `opt-level = "z"`: aggressive size optimization
- `lto = true`: link-time optimization
- `codegen-units = 1`: better optimization
- `strip = true`: strip debug symbols
- `panic = "abort"`: smaller binary by aborting on panic

Release binary built successfully (538K). Build with `cargo build --release`.

---

## User

Explain the #înline* annotation in Rust

---

## Assistant (Build · mimo-v2-omni-free · 4.0s)

_Thinking:_

The user asks about the `#inline*` annotation in Rust. They likely mean the `#[inline]` attribute. Need to explain what it does, its variants (`#[inline]`, `#[inline(always)]`, `#[inline(never)]`), and when to use it. Provide a concise explanation. No tools needed. Let's respond.

The `#[inline]` attribute in Rust suggests to the compiler whether to inline a function. 

- `#[inline]`: Encourages inlining, but compiler decides.
- `#[inline(always)]`: Always inline (if possible), used for small hot functions.
- `#[inline(never)]`: Never inline, useful for large functions or to avoid code bloat.

In practice, Rust's compiler (LLVM) already makes good inlining decisions. Use `#[inline]` for small, frequently called functions (like getters) and `#[inline(always)]` only for critical hot paths. `#[inline(never)]` is rarely needed but can help with compile times or debugging.

---

## User

run "explain #[inline] Rust annotation"

---

## Assistant (Build · mimo-v2-omni-free · 4.5s)

_Thinking:_

The user wants me to run an explanation of the #[inline] Rust annotation. They might be asking for a more detailed explanation or perhaps to run a search tool. But they said "run" which might mean they want me to execute something. However, I can't run a command that explains Rust annotations; I need to provide the explanation myself. I already gave a brief explanation. Maybe they want me to search the web or use the codesearch tool to get more information. Let me use the codesearch tool to get comprehensive information about #[inline] in Rust. That would be a good way to provide a thorough explanation. Let's do that.

**Tool: codesearch**

**Input:**
```json
{
  "query": "Rust #[inline] annotation attribute usage examples",
  "tokensNum": 3000
}
```

**Output:**
```
Title: Code generation - The Rust Reference
URL: https://doc.rust-lang.org/reference/attributes/codegen.html?highlight=inline
Code/Highlights:
Code generation - The Rust Reference

## Keyboard shortcuts

Press ← or → to navigate between chapters Press S or / to search in the book

Press ? to show this help

Press Esc to hide this help - Ayu
- Navy
- Coal
- Rust
- Light
- Auto

# The Rust Reference

[attributes.codegen]

# Code generation

---

Title: When to #[inline] - Standard library developers Guide
URL: https://std-dev-guide.rust-lang.org/policy/inline.html
Code/Highlights:
When to #[inline] - Standard library developers Guide 

- Light
- Rust
- Coal
- Navy
- Ayu # Standard library developers Guide

 

# When to #[inline] Inlining is a trade-off between potential execution speed, compile time and code size. There's some discussion about it in [this PR to the hashbrow

---

Title: When should inline be used in Rust?
URL: https://stackoverflow.com/questions/37639276/when-should-inline-be-used-in-rust
Code/Highlights:
# When should inline be used in Rust? - Tags: rust, inline, llvm-codegen
- Score: 142
- Views: 36,379
- Answers: 1
- Asked by: WiSaGaN (48,397 rep) - Asked on: Jun 5, 2016
- Last active: Sep 30, 2022
- License: CC BY-SA 4.0 ---

## Question

Rust has an "inline" attribute that can be used in one o

---

Title: Inline In Rust
URL: https://matklad.github.io/2021/07/09/inline-in-rust.html
Code/Highlights:
Inline In Rust There’s a lot of tribal knowledge surrounding [#[inline]] attribute in Rust. I often find myself teaching how it works, so I finally decided to write this down. Caveat Emptor: this is what I know, not

---

Title: What does #[inline] do?
URL: https://stackoverflow.com/questions/78382993/what-does-inline-do
Code/Highlights:
# What does #[inline] do? - Tags: rust
- Score: 0
- Views: 133
- Answers: 1
- Asked by: Umberto Fontanazza (658 rep)
- Asked on: Apr 25, 2024 - Last active: Apr 25, 2024
- License: CC BY-SA 4.0 ---

## Question

I have read [The rust performance book](https://nnethercote.github.io/perf-book/inlini

---

Title: Does const fn imply #[inline]?
URL: https://users.rust-lang.org/t/does-const-fn-imply-inline/74313
Code/Highlights:
Does const fn imply #[inline]? - help - The Rust Programming Language Forum
Skip to main content [The Rust Programming Language Forum](https://users.rust-lang.o

---

Title: Is it possible to set a function to only be inlined during a release build?
URL: https://stackoverflow.com/questions/41515342/is-it-possible-to-set-a-function-to-only-be-inlined-during-a-release-build
Code/Highlights:
# Is it possible to set a function to only be inlined during a release build? - Tags: attributes, rust, inline, conditional-compilation
- Score: 3
- Views: 540
- Answers: 1 - Asked by: NeuralSandwich (57 rep)
- Asked on: Jan 6, 2017
- Last active: Jan 6, 2017
- License: CC BY-SA 3.0 ---

## Questi

---

Title: InlineAttr in rustc_hir::attrs::data_structures - Rust
URL: https://doc.rust-lang.org/beta/nightly-rustc/rustc_hir/attrs/data_structures/enum.InlineAttr.html
Code/Highlights:
InlineAttr in rustc_hir::attrs::data_structures - RustSkip to main content

# Enum InlineAttr

Source ```
pub enum InlineAttr {
    None,
    Hint,
    Always,
    Never,
    Force {
        attr_spa
```

---

## Assistant (Build · mimo-v2-omni-free · 4.8s)

_Thinking:_

Now I have several relevant resources. I should extract key information to provide a comprehensive explanation. Let's fetch the first link, "Code generation - The Rust Reference", to get official details. Also the "When to #[inline] - Standard library developers Guide" for best practices. Let's fetch those.

**Tool: webfetch**

**Input:**
```json
{
  "url": "https://doc.rust-lang.org/reference/attributes/codegen.html?highlight=inline",
  "format": "markdown"
}
```

**Output:**
```
 Code generation - The Rust Reference                   

## Keyboard shortcuts

Press ← or → to navigate between chapters

Press S or / to search in the book

Press ? to show this help

Press Esc to hide this help

    

-   Auto
-   Light
-   Rust
-   Coal
-   Navy
-   Ayu

# The Rust Reference

[](../print.html "Print this book")[](https://github.com/rust-lang/reference/ "Git repository")[](https://github.com/rust-lang/reference/edit/master/src/attributes/codegen.md "Suggest an edit")

[\[attributes.codegen\]](#r-attributes.codegen "attributes.codegen")

# [Code generation attributes](#code-generation-attributes)

The following [attributes](../attributes.html) are used for controlling code generation.

[\[attributes.codegen.inline\]](#r-attributes.codegen.inline "attributes.codegen.inline")

### [The `inline` attribute](#the-inline-attribute)

[\[attributes.codegen.inline.intro\]](#r-attributes.codegen.inline.intro "attributes.codegen.inline.intro")

The *`inline` [attribute](../attributes.html)* suggests whether a copy of the attributed function’s code should be placed in the caller rather than generating a call to the function.

> Example
> 
> ```rust
> #![allow(unused)]
> fn main() {
> #[inline]
> pub fn example1() {}
> 
> #[inline(always)]
> pub fn example2() {}
> 
> #[inline(never)]
> pub fn example3() {}
> }
> ```

> Note
> 
> `rustc` automatically inlines functions when doing so seems worthwhile. Use this attribute carefully as poor decisions about what to inline can slow down programs.

[\[attributes.codegen.inline.syntax\]](#r-attributes.codegen.inline.syntax "attributes.codegen.inline.syntax")

The syntax for the `inline` attribute is:

**Syntax**  
[InlineAttribute](codegen.html#railroad-InlineAttribute) →  
      inline ( always )  
    | inline ( never )  
    | inline

Show Railroad

InlineAttribute inline ( always ) inline ( never ) inline

[\[attributes.codegen.inline.allowed-positions\]](#r-attributes.codegen.inline.allowed-positions "attributes.codegen.inline.allowed-positions")

The `inline` attribute may only be applied to functions with [bodies](../items/functions.html#r-items.fn.body) — [closures](../expressions/closure-expr.html#r-expr.closure), [async blocks](../expressions/block-expr.html#r-expr.block.async), [free functions](../items/functions.html#r-items.fn), [associated functions](../items/associated-items.html#r-items.associated.fn) in an [inherent impl](../items/implementations.html#r-items.impl.inherent) or [trait impl](../items/implementations.html#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.html#r-items.traits) when those functions have a [default definition](../items/traits.html#r-items.traits.associated-item-decls) .

> Note
> 
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> Note
> 
> Though the attribute can be applied to [closures](../expressions/closure-expr.html#r-expr.closure) and [async blocks](../expressions/block-expr.html#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.
> 
> ```rust
> #![allow(unused)]
> fn main() {
> // We allow attributes on statements.
> #[inline] || (); // OK
> #[inline] async {}; // OK
> }
> ```
> 
> ```rust
> #![allow(unused)]
> fn main() {
> // We don't yet allow attributes on expressions.
> let f = #[inline] || (); // ERROR
> }
> ```

[\[attributes.codegen.inline.duplicates\]](#r-attributes.codegen.inline.duplicates "attributes.codegen.inline.duplicates")

Only the first use of `inline` on a function has effect.

> Note
> 
> `rustc` lints against any use following the first. This may become an error in the future.

[\[attributes.codegen.inline.modes\]](#r-attributes.codegen.inline.modes "attributes.codegen.inline.modes")

The `inline` attribute supports these modes:

-   `#[inline]` *suggests* performing inline expansion.
-   `#[inline(always)]` *suggests* that inline expansion should always be performed.
-   `#[inline(never)]` *suggests* that inline expansion should never be performed.

> Note
> 
> In every form the attribute is a hint. The compiler may ignore it.

[\[attributes.codegen.inline.trait\]](#r-attributes.codegen.inline.trait "attributes.codegen.inline.trait")

When `inline` is applied to a function in a [trait](../items/traits.html#r-items.traits), it applies only to the code of the [default definition](../items/traits.html#r-items.traits.associated-item-decls).

[\[attributes.codegen.inline.async\]](#r-attributes.codegen.inline.async "attributes.codegen.inline.async")

When `inline` is applied to an [async function](../items/functions.html#r-items.fn.async) or [async closure](../expressions/closure-expr.html#r-expr.closure.async), it applies only to the code of the generated `poll` function.

> Note
> 
> For more details, see [Rust issue #129347](https://github.com/rust-lang/rust/issues/129347).

[\[attributes.codegen.inline.externally-exported\]](#r-attributes.codegen.inline.externally-exported "attributes.codegen.inline.externally-exported")

The `inline` attribute is ignored if the function is externally exported with [`no_mangle`](../abi.html#r-abi.no_mangle) or [`export_name`](../abi.html#r-abi.export_name).

[\[attributes.codegen.cold\]](#r-attributes.codegen.cold "attributes.codegen.cold")

### [The `cold` attribute](#the-cold-attribute)

[\[attributes.codegen.cold.intro\]](#r-attributes.codegen.cold.intro "attributes.codegen.cold.intro")  

[Tests](javascript:void\(0\))

Tests with this rule:

-   [tests/codegen-llvm/cold-attribute.rs](https://github.com/rust-lang/rust/blob/1.94.1/tests/codegen-llvm/cold-attribute.rs)

The *`cold` [attribute](../attributes.html)* suggests that the attributed function is unlikely to be called which may help the compiler produce better code.

> Example
> 
> ```rust
> #![allow(unused)]
> fn main() {
> #[cold]
> pub fn example() {}
> }
> ```

[\[attributes.codegen.cold.syntax\]](#r-attributes.codegen.cold.syntax "attributes.codegen.cold.syntax")

The `cold` attribute uses the [MetaWord](../attributes.html#grammar-MetaWord) syntax.

[\[attributes.codegen.cold.allowed-positions\]](#r-attributes.codegen.cold.allowed-positions "attributes.codegen.cold.allowed-positions")

The `cold` attribute may only be applied to functions with [bodies](../items/functions.html#r-items.fn.body) — [closures](../expressions/closure-expr.html#r-expr.closure), [async blocks](../expressions/block-expr.html#r-expr.block.async), [free functions](../items/functions.html#r-items.fn), [associated functions](../items/associated-items.html#r-items.associated.fn) in an [inherent impl](../items/implementations.html#r-items.impl.inherent) or [trait impl](../items/implementations.html#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.html#r-items.traits) when those functions have a [default definition](../items/traits.html#r-items.traits.associated-item-decls) .

> Note
> 
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> Note
> 
> Though the attribute can be applied to [closures](../expressions/closure-expr.html#r-expr.closure) and [async blocks](../expressions/block-expr.html#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.

[\[attributes.codegen.cold.duplicates\]](#r-attributes.codegen.cold.duplicates "attributes.codegen.cold.duplicates")

Only the first use of `cold` on a function has effect.

> Note
> 
> `rustc` lints against any use following the first. This may become an error in the future.

[\[attributes.codegen.cold.trait\]](#r-attributes.codegen.cold.trait "attributes.codegen.cold.trait")  

[Tests](javascript:void\(0\))

Tests with this rule:

-   [tests/codegen-llvm/cold-attribute.rs](https://github.com/rust-lang/rust/blob/1.94.1/tests/codegen-llvm/cold-attribute.rs)

When `cold` is applied to a function in a [trait](../items/traits.html#r-items.traits), it applies only to the code of the [default definition](../items/traits.html#r-items.traits.associated-item-decls).

[\[attributes.codegen.naked\]](#r-attributes.codegen.naked "attributes.codegen.naked")

## [The `naked` attribute](#the-naked-attribute)

[\[attributes.codegen.naked.intro\]](#r-attributes.codegen.naked.intro "attributes.codegen.naked.intro")

The *`naked` [attribute](../attributes.html)* prevents the compiler from emitting a function prologue and epilogue for the attributed function.

[\[attributes.codegen.naked.body\]](#r-attributes.codegen.naked.body "attributes.codegen.naked.body")

The [function body](../items/functions.html#function-body) must consist of exactly one [`naked_asm!`](../inline-assembly.html) macro invocation.

[\[attributes.codegen.naked.prologue-epilogue\]](#r-attributes.codegen.naked.prologue-epilogue "attributes.codegen.naked.prologue-epilogue")

No function prologue or epilogue is generated for the attributed function. The assembly code in the `naked_asm!` block constitutes the full body of a naked function.

[\[attributes.codegen.naked.unsafe-attribute\]](#r-attributes.codegen.naked.unsafe-attribute "attributes.codegen.naked.unsafe-attribute")

The `naked` attribute is an [unsafe attribute](../attributes.html#r-attributes.safety). Annotating a function with `#[unsafe(naked)]` comes with the safety obligation that the body must respect the function’s calling convention, uphold its signature, and either return or diverge (i.e., not fall through past the end of the assembly code).

[\[attributes.codegen.naked.call-stack\]](#r-attributes.codegen.naked.call-stack "attributes.codegen.naked.call-stack")

The assembly code may assume that the call stack and register state are valid on entry as per the signature and calling convention of the function.

[\[attributes.codegen.naked.no-duplication\]](#r-attributes.codegen.naked.no-duplication "attributes.codegen.naked.no-duplication")

The assembly code may not be duplicated by the compiler except when monomorphizing polymorphic functions.

> Note
> 
> Guaranteeing when the assembly code may or may not be duplicated is important for naked functions that define symbols.

[\[attributes.codegen.naked.unused-variables\]](#r-attributes.codegen.naked.unused-variables "attributes.codegen.naked.unused-variables")

The [`unused_variables`](../../rustc/lints/listing/warn-by-default.html#unused-variables) lint is suppressed within naked functions.

[\[attributes.codegen.naked.inline\]](#r-attributes.codegen.naked.inline "attributes.codegen.naked.inline")

The [`inline`](#the-inline-attribute) attribute cannot by applied to a naked function.

[\[attributes.codegen.naked.track\_caller\]](#r-attributes.codegen.naked.track_caller "attributes.codegen.naked.track_caller")

The [`track_caller`](#the-track_caller-attribute) attribute cannot be applied to a naked function.

[\[attributes.codegen.naked.testing\]](#r-attributes.codegen.naked.testing "attributes.codegen.naked.testing")

The [testing attributes](testing.html) cannot be applied to a naked function.

[\[attributes.codegen.no\_builtins\]](#r-attributes.codegen.no_builtins "attributes.codegen.no_builtins")

## [The `no_builtins` attribute](#the-no_builtins-attribute)

[\[attributes.codegen.no\_builtins.intro\]](#r-attributes.codegen.no_builtins.intro "attributes.codegen.no_builtins.intro")

The *`no_builtins` [attribute](../attributes.html)* disables optimization of certain code patterns related to calls to library functions that are assumed to exist.

> Example
> 
> ```rust
> #![allow(unused)]
> #![no_builtins]
> fn main() {
> }
> ```

[\[attributes.codegen.no\_builtins.syntax\]](#r-attributes.codegen.no_builtins.syntax "attributes.codegen.no_builtins.syntax")

The `no_builtins` attribute uses the [MetaWord](../attributes.html#grammar-MetaWord) syntax.

[\[attributes.codegen.no\_builtins.allowed-positions\]](#r-attributes.codegen.no_builtins.allowed-positions "attributes.codegen.no_builtins.allowed-positions")

The `no_builtins` attribute can only be applied to the crate root.

[\[attributes.codegen.no\_builtins.duplicates\]](#r-attributes.codegen.no_builtins.duplicates "attributes.codegen.no_builtins.duplicates")

Only the first use of the `no_builtins` attribute has effect.

> Note
> 
> `rustc` lints against any use following the first.

[\[attributes.codegen.target\_feature\]](#r-attributes.codegen.target_feature "attributes.codegen.target_feature")

## [The `target_feature` attribute](#the-target_feature-attribute)

[\[attributes.codegen.target\_feature.intro\]](#r-attributes.codegen.target_feature.intro "attributes.codegen.target_feature.intro")

The *`target_feature` [attribute](../attributes.html)* may be applied to a function to enable code generation of that function for specific platform architecture features. It uses the [MetaListNameValueStr](../attributes.html#grammar-MetaListNameValueStr) syntax with a single key of `enable` whose value is a string of comma-separated feature names to enable.

```rust
#![allow(unused)]
fn main() {
#[cfg(target_feature = "avx2")]
#[target_feature(enable = "avx2")]
fn foo_avx2() {}
}
```

[\[attributes.codegen.target\_feature.arch\]](#r-attributes.codegen.target_feature.arch "attributes.codegen.target_feature.arch")

Each [target architecture](../conditional-compilation.html#target_arch) has a set of features that may be enabled. It is an error to specify a feature for a target architecture that the crate is not being compiled for.

[\[attributes.codegen.target\_feature.closures\]](#r-attributes.codegen.target_feature.closures "attributes.codegen.target_feature.closures")

Closures defined within a `target_feature`\-annotated function inherit the attribute from the enclosing function.

[\[attributes.codegen.target\_feature.target-ub\]](#r-attributes.codegen.target_feature.target-ub "attributes.codegen.target_feature.target-ub")

It is [undefined behavior](../behavior-considered-undefined.html) to call a function that is compiled with a feature that is not supported on the current platform the code is running on, *except* if the platform explicitly documents this to be safe.

[\[attributes.codegen.target\_feature.safety-restrictions\]](#r-attributes.codegen.target_feature.safety-restrictions "attributes.codegen.target_feature.safety-restrictions")

The following restrictions apply unless otherwise specified by the platform rules below:

-   Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be safely called within a caller that enables all the `target_feature`s that the callee enables. This restriction does not apply in an `unsafe` context.
-   Safe `#[target_feature]` functions (and closures that inherit the attribute) can only be coerced to *safe* function pointers in contexts that enable all the `target_feature`s that the coercee enables. This restriction does not apply to `unsafe` function pointers.

Implicitly enabled features are included in this rule. For example an `sse2` function can call ones marked with `sse`.

```rust
#![allow(unused)]
fn main() {
#[cfg(target_feature = "sse2")] {
#[target_feature(enable = "sse")]
fn foo_sse() {}

fn bar() {
    // Calling `foo_sse` here is unsafe, as we must ensure that SSE is
    // available first, even if `sse` is enabled by default on the target
    // platform or manually enabled as compiler flags.
    unsafe {
        foo_sse();
    }
}

#[target_feature(enable = "sse")]
fn bar_sse() {
    // Calling `foo_sse` here is safe.
    foo_sse();
    || foo_sse();
}

#[target_feature(enable = "sse2")]
fn bar_sse2() {
    // Calling `foo_sse` here is safe because `sse2` implies `sse`.
    foo_sse();
}
}
}
```

[\[attributes.codegen.target\_feature.fn-traits\]](#r-attributes.codegen.target_feature.fn-traits "attributes.codegen.target_feature.fn-traits")

A function with a `#[target_feature]` attribute *never* implements the `Fn` family of traits, although closures inheriting features from the enclosing function do.

[\[attributes.codegen.target\_feature.allowed-positions\]](#r-attributes.codegen.target_feature.allowed-positions "attributes.codegen.target_feature.allowed-positions")

The `#[target_feature]` attribute is not allowed on the following places:

-   [the `main` function](../crates-and-source-files.html#r-crate.main)
-   a [`panic_handler` function](../panic.html#r-panic.panic_handler)
-   safe trait methods
-   safe default functions in traits

[\[attributes.codegen.target\_feature.inline\]](#r-attributes.codegen.target_feature.inline "attributes.codegen.target_feature.inline")

Functions marked with `target_feature` are not inlined into a context that does not support the given features. The `#[inline(always)]` attribute may not be used with a `target_feature` attribute.

[\[attributes.codegen.target\_feature.availability\]](#r-attributes.codegen.target_feature.availability "attributes.codegen.target_feature.availability")

### [Available features](#available-features)

The following is a list of the available feature names.

[\[attributes.codegen.target\_feature.x86\]](#r-attributes.codegen.target_feature.x86 "attributes.codegen.target_feature.x86")

#### [`x86` or `x86_64`](#x86-or-x86_64)

Executing code with unsupported features is undefined behavior on this platform. Hence on this platform usage of `#[target_feature]` functions follows the [above restrictions](codegen.html#r-attributes.codegen.target_feature.safety-restrictions).

Feature

Implicitly Enables

Description

`adx`

[ADX](https://en.wikipedia.org/wiki/Intel_ADX) — Multi-Precision Add-Carry Instruction Extensions

`aes`

`sse2`

[AES](https://en.wikipedia.org/wiki/AES_instruction_set) — Advanced Encryption Standard

`avx`

`sse4.2`

[AVX](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions) — Advanced Vector Extensions

`avx2`

`avx`

[AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX2) — Advanced Vector Extensions 2

`avx512bf16`

`avx512bw`

[AVX512-BF16](https://en.wikipedia.org/wiki/AVX-512#BF16) — Advanced Vector Extensions 512-bit - Bfloat16 Extensions

`avx512bitalg`

`avx512bw`

[AVX512-BITALG](https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG) — Advanced Vector Extensions 512-bit - Bit Algorithms

`avx512bw`

`avx512f`

[AVX512-BW](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Byte and Word Instructions

`avx512cd`

`avx512f`

[AVX512-CD](https://en.wikipedia.org/wiki/AVX-512#Conflict_detection) — Advanced Vector Extensions 512-bit - Conflict Detection Instructions

`avx512dq`

`avx512f`

[AVX512-DQ](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Doubleword and Quadword Instructions

`avx512f`

`avx2`, `fma`, `f16c`

[AVX512-F](https://en.wikipedia.org/wiki/AVX-512) — Advanced Vector Extensions 512-bit - Foundation

`avx512fp16`

`avx512bw`

[AVX512-FP16](https://en.wikipedia.org/wiki/AVX-512#FP16) — Advanced Vector Extensions 512-bit - Float16 Extensions

`avx512ifma`

`avx512f`

[AVX512-IFMA](https://en.wikipedia.org/wiki/AVX-512#IFMA) — Advanced Vector Extensions 512-bit - Integer Fused Multiply Add

`avx512vbmi`

`avx512bw`

[AVX512-VBMI](https://en.wikipedia.org/wiki/AVX-512#BW,_DQ_and_VBMI) — Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions

`avx512vbmi2`

`avx512bw`

[AVX512-VBMI2](https://en.wikipedia.org/wiki/AVX-512#VBMI2) — Advanced Vector Extensions 512-bit - Vector Byte Manipulation Instructions 2

`avx512vl`

`avx512f`

[AVX512-VL](https://en.wikipedia.org/wiki/AVX-512) — Advanced Vector Extensions 512-bit - Vector Length Extensions

`avx512vnni`

`avx512f`

[AVX512-VNNI](https://en.wikipedia.org/wiki/AVX-512#VNNI) — Advanced Vector Extensions 512-bit - Vector Neural Network Instructions

`avx512vp2intersect`

`avx512f`

[AVX512-VP2INTERSECT](https://en.wikipedia.org/wiki/AVX-512#VP2INTERSECT) — Advanced Vector Extensions 512-bit - Vector Pair Intersection to a Pair of Mask Registers

`avx512vpopcntdq`

`avx512f`

[AVX512-VPOPCNTDQ](https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG) — Advanced Vector Extensions 512-bit - Vector Population Count Instruction

`avxifma`

`avx2`

[AVX-IFMA](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Integer Fused Multiply Add

`avxneconvert`

`avx2`

[AVX-NE-CONVERT](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - No-Exception Floating-Point conversion Instructions

`avxvnni`

`avx2`

[AVX-VNNI](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions

`avxvnniint16`

`avx2`

[AVX-VNNI-INT16](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions with 16-bit Integers

`avxvnniint8`

`avx2`

[AVX-VNNI-INT8](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#AVX-VNNI,_AVX-IFMA) — Advanced Vector Extensions - Vector Neural Network Instructions with 8-bit Integers

`bmi1`

[BMI1](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets) — Bit Manipulation Instruction Sets

`bmi2`

[BMI2](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2) — Bit Manipulation Instruction Sets 2

`cmpxchg16b`

[`cmpxchg16b`](https://www.felixcloutier.com/x86/cmpxchg8b:cmpxchg16b) — Compares and exchange 16 bytes (128 bits) of data atomically

`f16c`

`avx`

[F16C](https://en.wikipedia.org/wiki/F16C) — 16-bit floating point conversion instructions

`fma`

`avx`

[FMA3](https://en.wikipedia.org/wiki/FMA_instruction_set) — Three-operand fused multiply-add

`fxsr`

[`fxsave`](https://www.felixcloutier.com/x86/fxsave) and [`fxrstor`](https://www.felixcloutier.com/x86/fxrstor) — Save and restore x87 FPU, MMX Technology, and SSE State

`gfni`

`sse2`

[GFNI](https://en.wikipedia.org/wiki/AVX-512#GFNI) — Galois Field New Instructions

`kl`

`sse2`

[KEYLOCKER](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions) — Intel Key Locker Instructions

`lzcnt`

[`lzcnt`](https://www.felixcloutier.com/x86/lzcnt) — Leading zeros count

`movbe`

[`movbe`](https://www.felixcloutier.com/x86/movbe) — Move data after swapping bytes

`pclmulqdq`

`sse2`

[`pclmulqdq`](https://www.felixcloutier.com/x86/pclmulqdq) — Packed carry-less multiplication quadword

`popcnt`

[`popcnt`](https://www.felixcloutier.com/x86/popcnt) — Count of bits set to 1

`rdrand`

[`rdrand`](https://en.wikipedia.org/wiki/RdRand) — Read random number

`rdseed`

[`rdseed`](https://en.wikipedia.org/wiki/RdRand) — Read random seed

`sha`

`sse2`

[SHA](https://en.wikipedia.org/wiki/Intel_SHA_extensions) — Secure Hash Algorithm

`sha512`

`avx2`

[SHA512](https://en.wikipedia.org/wiki/Intel_SHA_extensions) — Secure Hash Algorithm with 512-bit digest

`sm3`

`avx`

[SM3](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions) — ShangMi 3 Hash Algorithm

`sm4`

`avx2`

[SM4](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_SHA_and_SM3_instructions) — ShangMi 4 Cipher Algorithm

`sse`

[SSE](https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions) — Streaming SIMD Extensions

`sse2`

`sse`

[SSE2](https://en.wikipedia.org/wiki/SSE2) — Streaming SIMD Extensions 2

`sse3`

`sse2`

[SSE3](https://en.wikipedia.org/wiki/SSE3) — Streaming SIMD Extensions 3

`sse4.1`

`ssse3`

[SSE4.1](https://en.wikipedia.org/wiki/SSE4#SSE4.1) — Streaming SIMD Extensions 4.1

`sse4.2`

`sse4.1`

[SSE4.2](https://en.wikipedia.org/wiki/SSE4#SSE4.2) — Streaming SIMD Extensions 4.2

`sse4a`

`sse3`

[SSE4a](https://en.wikipedia.org/wiki/SSE4#SSE4a) — Streaming SIMD Extensions 4a

`ssse3`

`sse3`

[SSSE3](https://en.wikipedia.org/wiki/SSSE3) — Supplemental Streaming SIMD Extensions 3

`tbm`

[TBM](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#TBM_\(Trailing_Bit_Manipulation\)) — Trailing Bit Manipulation

`vaes`

`avx2`, `aes`

[VAES](https://en.wikipedia.org/wiki/AVX-512#VAES) — Vector AES Instructions

`vpclmulqdq`

`avx`, `pclmulqdq`

[VPCLMULQDQ](https://en.wikipedia.org/wiki/AVX-512#VPCLMULQDQ) — Vector Carry-less multiplication of Quadwords

`widekl`

`kl`

[KEYLOCKER\_WIDE](https://en.wikipedia.org/wiki/List_of_x86_cryptographic_instructions#Intel_Key_Locker_instructions) — Intel Wide Keylocker Instructions

`xsave`

[`xsave`](https://www.felixcloutier.com/x86/xsave) — Save processor extended states

`xsavec`

[`xsavec`](https://www.felixcloutier.com/x86/xsavec) — Save processor extended states with compaction

`xsaveopt`

[`xsaveopt`](https://www.felixcloutier.com/x86/xsaveopt) — Save processor extended states optimized

`xsaves`

[`xsaves`](https://www.felixcloutier.com/x86/xsaves) — Save processor extended states supervisor

[\[attributes.codegen.target\_feature.aarch64\]](#r-attributes.codegen.target_feature.aarch64 "attributes.codegen.target_feature.aarch64")

#### [`aarch64`](#aarch64)

On this platform the usage of `#[target_feature]` functions follows the [above restrictions](codegen.html#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in the [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest), or elsewhere on [developer.arm.com](https://developer.arm.com).

> Note
> 
> The following pairs of features should both be marked as enabled or disabled together if used:
> 
> -   `paca` and `pacg`, which LLVM currently implements as one feature.

Feature

Implicitly Enables

Feature Name

`aes`

`neon`

FEAT\_AES & FEAT\_PMULL — Advanced SIMD AES & PMULL instructions

`bf16`

FEAT\_BF16 — BFloat16 instructions

`bti`

FEAT\_BTI — Branch Target Identification

`crc`

FEAT\_CRC — CRC32 checksum instructions

`dit`

FEAT\_DIT — Data Independent Timing instructions

`dotprod`

`neon`

FEAT\_DotProd — Advanced SIMD Int8 dot product instructions

`dpb`

FEAT\_DPB — Data cache clean to point of persistence

`dpb2`

`dpb`

FEAT\_DPB2 — Data cache clean to point of deep persistence

`f32mm`

`sve`

FEAT\_F32MM — SVE single-precision FP matrix multiply instruction

`f64mm`

`sve`

FEAT\_F64MM — SVE double-precision FP matrix multiply instruction

`fcma`

`neon`

FEAT\_FCMA — Floating point complex number support

`fhm`

`fp16`

FEAT\_FHM — Half-precision FP FMLAL instructions

`flagm`

FEAT\_FLAGM — Conditional flag manipulation

`fp16`

`neon`

FEAT\_FP16 — Half-precision FP data processing

`frintts`

FEAT\_FRINTTS — Floating-point to int helper instructions

`i8mm`

FEAT\_I8MM — Int8 Matrix Multiplication

`jsconv`

`neon`

FEAT\_JSCVT — JavaScript conversion instruction

`lor`

FEAT\_LOR — Limited Ordering Regions extension

`lse`

FEAT\_LSE — Large System Extensions

`mte`

FEAT\_MTE & FEAT\_MTE2 — Memory Tagging Extension

`neon`

FEAT\_AdvSimd & FEAT\_FP — Floating Point and Advanced SIMD extension

`paca`

FEAT\_PAUTH — Pointer Authentication (address authentication)

`pacg`

FEAT\_PAUTH — Pointer Authentication (generic authentication)

`pan`

FEAT\_PAN — Privileged Access-Never extension

`pmuv3`

FEAT\_PMUv3 — Performance Monitors extension (v3)

`rand`

FEAT\_RNG — Random Number Generator

`ras`

FEAT\_RAS & FEAT\_RASv1p1 — Reliability, Availability and Serviceability extension

`rcpc`

FEAT\_LRCPC — Release consistent Processor Consistent

`rcpc2`

`rcpc`

FEAT\_LRCPC2 — RcPc with immediate offsets

`rdm`

`neon`

FEAT\_RDM — Rounding Double Multiply accumulate

`sb`

FEAT\_SB — Speculation Barrier

`sha2`

`neon`

FEAT\_SHA1 & FEAT\_SHA256 — Advanced SIMD SHA instructions

`sha3`

`sha2`

FEAT\_SHA512 & FEAT\_SHA3 — Advanced SIMD SHA instructions

`sm4`

`neon`

FEAT\_SM3 & FEAT\_SM4 — Advanced SIMD SM3/4 instructions

`spe`

FEAT\_SPE — Statistical Profiling Extension

`ssbs`

FEAT\_SSBS & FEAT\_SSBS2 — Speculative Store Bypass Safe

`sve`

`neon`

FEAT\_SVE — Scalable Vector Extension

`sve2`

`sve`

FEAT\_SVE2 — Scalable Vector Extension 2

`sve2-aes`

`sve2`, `aes`

FEAT\_SVE\_AES & FEAT\_SVE\_PMULL128 — SVE AES instructions

`sve2-bitperm`

`sve2`

FEAT\_SVE2\_BitPerm — SVE Bit Permute

`sve2-sha3`

`sve2`, `sha3`

FEAT\_SVE2\_SHA3 — SVE SHA3 instructions

`sve2-sm4`

`sve2`, `sm4`

FEAT\_SVE2\_SM4 — SVE SM4 instructions

`tme`

FEAT\_TME — Transactional Memory Extension

`vh`

FEAT\_VHE — Virtualization Host Extensions

[\[attributes.codegen.target\_feature.loongarch\]](#r-attributes.codegen.target_feature.loongarch "attributes.codegen.target_feature.loongarch")

#### [`loongarch`](#loongarch)

On this platform the usage of `#[target_feature]` functions follows the [above restrictions](codegen.html#r-attributes.codegen.target_feature.safety-restrictions).

Feature

Implicitly Enables

Description

`f`

[F](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_sp) — Single-precision float-point instructions

`d`

`f`

[D](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-fp_dp) — Double-precision float-point instructions

`frecipe`

[FRECIPE](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-frecipe) — Reciprocal approximation instructions

`lasx`

`lsx`

[LASX](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lasx) — 256-bit vector instructions

`lbt`

[LBT](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lbt_x86) — Binary translation instructions

`lsx`

`d`

[LSX](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lsx) — 128-bit vector instructions

`lvz`

[LVZ](https://loongson.github.io/LoongArch-Documentation/LoongArch-Vol1-EN.html#cpucfg-lvz) — Virtualization instructions

[\[attributes.codegen.target\_feature.riscv\]](#r-attributes.codegen.target_feature.riscv "attributes.codegen.target_feature.riscv")

#### [`riscv32` or `riscv64`](#riscv32-or-riscv64)

On this platform the usage of `#[target_feature]` functions follows the [above restrictions](codegen.html#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in their respective specification. Many specifications are described in the [RISC-V ISA Manual](https://github.com/riscv/riscv-isa-manual), [version 20250508](https://github.com/riscv/riscv-isa-manual/tree/20250508), or in another manual hosted on the [RISC-V GitHub Account](https://github.com/riscv).

Feature

Implicitly Enables

Description

`a`

`zaamo`, `zalrsc`

[A](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Atomic instructions

`b`

`zba`, `zbc`, `zbs`

[B](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Bit Manipulation instructions

`c`

`zca`

[C](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/c-st-ext.adoc) — Compressed instructions

`m`

[M](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/m-st-ext.adoc) — Integer Multiplication and Division instructions

`za64rs`

`za128rs`

[Za64rs](https://github.com/riscv/riscv-profiles/blob/rva23-rvb23-ratified/src/rva23-profile.adoc) — Platform Behavior: Naturally aligned Reservation sets with ≦ 64 Bytes

`za128rs`

[Za128rs](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Naturally aligned Reservation sets with ≦ 128 Bytes

`zaamo`

[Zaamo](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Atomic Memory Operation instructions

`zabha`

`zaamo`

[Zabha](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zabha.adoc) — Byte and Halfword Atomic Memory Operation instructions

`zacas`

`zaamo`

[Zacas](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zacas.adoc) — Atomic Compare-and-Swap (CAS) instructions

`zalrsc`

[Zalrsc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/a-st-ext.adoc) — Load-Reserved/Store-Conditional instructions

`zama16b`

[Zama16b](https://github.com/riscv/riscv-profiles/blob/rva23-rvb23-ratified/src/rva23-profile.adoc) — Platform Behavior: Misaligned loads, stores, and AMOs to main memory regions that do not cross a naturally aligned 16-byte boundary are atomic

`zawrs`

[Zawrs](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zawrs.adoc) — Wait-on-Reservation-Set instructions

`zba`

[Zba](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Address Generation instructions

`zbb`

[Zbb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Basic bit-manipulation

`zbc`

`zbkc`

[Zbc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Carry-less multiplication

`zbkb`

[Zbkb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Bit Manipulation Instructions for Cryptography

`zbkc`

[Zbkc](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Carry-less multiplication for Cryptography

`zbkx`

[Zbkx](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Crossbar permutations

`zbs`

[Zbs](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/b-st-ext.adoc) — Single-bit instructions

`zca`

[Zca](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zc.adoc) — Compressed instructions: integer part subset

`zcb`

`zca`

[Zcb](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zc.adoc) — Simple Code-size Saving Compressed instructions

`zcmop`

`zca`

[Zcmop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zimop.adoc) — Compressed May-Be-Operations

`zic64b`

[Zic64b](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Naturally aligned 64 byte Cache blocks

`zicbom`

[Zicbom](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Management instructions

`zicbop`

[Zicbop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Prefetch Hint instructions

`zicboz`

[Zicboz](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/cmo.adoc) — Cache-Block Zero instruction

`ziccamoa`

[Ziccamoa](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports all basic atomic operations

`ziccif`

[Ziccif](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports instruction fetch and fetches of naturally aligned power-of-2 sizes up to `min(ILEN,XLEN)` are atomic

`zicclsm`

[Zicclsm](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory supports misaligned load/store accesses

`ziccrse`

[Ziccrse](https://github.com/riscv/riscv-profiles/blob/v1.0/profiles.adoc) — Platform Behavior: Cacheable and Coherent Main memory guarantees eventual success on LR/SC sequences

`zicntr`

`zicsr`

[Zicntr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/counters.adoc) — Base Counters and Timers

`zicond`

[Zicond](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zicond.adoc) — Integer Conditional Operation instructions

`zicsr`

[Zicsr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zicsr.adoc) — Control and Status Register (CSR) instructions

`zifencei`

[Zifencei](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zifencei.adoc) — Instruction-Fetch Fence instruction

`zihintntl`

[Zihintntl](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zihintntl.adoc) — Non-Temporal Locality Hint instructions

`zihintpause`

[Zihintpause](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zihintpause.adoc) — Pause Hint instruction

`zihpm`

`zicsr`

[Zihpm](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/counters.adoc) — Hardware Performance Counters

`zimop`

[Zimop](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/zimop.adoc) — May-Be-Operations

`zk`

`zkn`, `zkr`, `zks`, `zkt`, `zbkb`, `zbkc`, `zkbx`

[Zk](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Scalar Cryptography

`zkn`

`zknd`, `zkne`, `zknh`, `zbkb`, `zbkc`, `zkbx`

[Zkn](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Algorithm suite extension

`zknd`

[Zknd](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: AES Decryption

`zkne`

[Zkne](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: AES Encryption

`zknh`

[Zknh](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — NIST Suite: Hash Function Instructions

`zkr`

[Zkr](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Entropy Source Extension

`zks`

`zksed`, `zksh`, `zbkb`, `zbkc`, `zkbx`

[Zks](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Algorithm Suite

`zksed`

[Zksed](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Suite: SM4 Block Cipher Instructions

`zksh`

[Zksh](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — ShangMi Suite: SM3 Hash Function Instructions

`zkt`

[Zkt](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/scalar-crypto.adoc) — Data Independent Execution Latency Subset

`ztso`

[Ztso](https://github.com/riscv/riscv-isa-manual/blob/20250508/src/ztso-st-ext.adoc) — Total Store Ordering

[\[attributes.codegen.target\_feature.wasm\]](#r-attributes.codegen.target_feature.wasm "attributes.codegen.target_feature.wasm")

#### [`wasm32` or `wasm64`](#wasm32-or-wasm64)

Safe `#[target_feature]` functions may always be used in safe contexts on Wasm platforms. It is impossible to cause undefined behavior via the `#[target_feature]` attribute because attempting to use instructions unsupported by the Wasm engine will fail at load time without the risk of being interpreted in a way different from what the compiler expected.

Feature

Implicitly Enables

Description

`bulk-memory`

[WebAssembly bulk memory operations proposal](https://github.com/WebAssembly/bulk-memory-operations)

`extended-const`

[WebAssembly extended const expressions proposal](https://github.com/WebAssembly/extended-const)

`mutable-globals`

[WebAssembly mutable global proposal](https://github.com/WebAssembly/mutable-global)

`nontrapping-fptoint`

[WebAssembly non-trapping float-to-int conversion proposal](https://github.com/WebAssembly/nontrapping-float-to-int-conversions)

`relaxed-simd`

`simd128`

[WebAssembly relaxed simd proposal](https://github.com/WebAssembly/relaxed-simd)

`sign-ext`

[WebAssembly sign extension operators Proposal](https://github.com/WebAssembly/sign-extension-ops)

`simd128`

[WebAssembly simd proposal](https://github.com/webassembly/simd)

`multivalue`

[WebAssembly multivalue proposal](https://github.com/webassembly/multi-value)

`reference-types`

[WebAssembly reference-types proposal](https://github.com/webassembly/reference-types)

`tail-call`

[WebAssembly tail-call proposal](https://github.com/webassembly/tail-call)

[\[attributes.codegen.target\_feature.s390x\]](#r-attributes.codegen.target_feature.s390x "attributes.codegen.target_feature.s390x")

#### [`s390x`](#s390x)

On `s390x` targets, use of functions with the `#[target_feature]` attribute follows the [above restrictions](codegen.html#r-attributes.codegen.target_feature.safety-restrictions).

Further documentation on these features can be found in the “Additions to z/Architecture” section of Chapter 1 of the *[z/Architecture Principles of Operation](https://publibfp.dhe.ibm.com/epubs/pdf/a227832d.pdf)*.

Feature

Implicitly Enables

Description

`vector`

128-bit vector instructions

`vector-enhancements-1`

`vector`

vector enhancements 1

`vector-enhancements-2`

`vector-enhancements-1`

vector enhancements 2

`vector-enhancements-3`

`vector-enhancements-2`

vector enhancements 3

`vector-packed-decimal`

`vector`

vector packed-decimal

`vector-packed-decimal-enhancement`

`vector-packed-decimal`

vector packed-decimal enhancement

`vector-packed-decimal-enhancement-2`

`vector-packed-decimal-enhancement-2`

vector packed-decimal enhancement 2

`vector-packed-decimal-enhancement-3`

`vector-packed-decimal-enhancement-3`

vector packed-decimal enhancement 3

`nnp-assist`

`vector`

nnp assist

`miscellaneous-extensions-2`

miscellaneous extensions 2

`miscellaneous-extensions-3`

miscellaneous extensions 3

`miscellaneous-extensions-4`

miscellaneous extensions 4

[\[attributes.codegen.target\_feature.info\]](#r-attributes.codegen.target_feature.info "attributes.codegen.target_feature.info")

### [Additional information](#additional-information)

[\[attributes.codegen.target\_feature.remark-cfg\]](#r-attributes.codegen.target_feature.remark-cfg "attributes.codegen.target_feature.remark-cfg")

See the [`target_feature` conditional compilation option](../conditional-compilation.html#target_feature) for selectively enabling or disabling compilation of code based on compile-time settings. Note that this option is not affected by the `target_feature` attribute, and is only driven by the features enabled for the entire crate.

[\[attributes.codegen.target\_feature.remark-rt\]](#r-attributes.codegen.target_feature.remark-rt "attributes.codegen.target_feature.remark-rt")

Whether a feature is enabled can be checked at runtime using a platform-specific macro from the standard library, for instance [`is_x86_feature_detected`](../../std/arch/macro.is_x86_feature_detected.html) or [`is_aarch64_feature_detected`](../../std/arch/macro.is_aarch64_feature_detected.html).

> Note
> 
> `rustc` has a default set of features enabled for each target and CPU. The CPU may be chosen with the [`-C target-cpu`](../../rustc/codegen-options/index.html#target-cpu) flag. Individual features may be enabled or disabled for an entire crate with the [`-C target-feature`](../../rustc/codegen-options/index.html#target-feature) flag.

[\[attributes.codegen.track\_caller\]](#r-attributes.codegen.track_caller "attributes.codegen.track_caller")

## [The `track_caller` attribute](#the-track_caller-attribute)

[\[attributes.codegen.track\_caller.allowed-positions\]](#r-attributes.codegen.track_caller.allowed-positions "attributes.codegen.track_caller.allowed-positions")

The `track_caller` attribute may be applied to any function with [`"Rust"` ABI](../items/external-blocks.html#abi) with the exception of the entry point `fn main`.

[\[attributes.codegen.track\_caller.traits\]](#r-attributes.codegen.track_caller.traits "attributes.codegen.track_caller.traits")

When applied to functions and methods in trait declarations, the attribute applies to all implementations. If the trait provides a default implementation with the attribute, then the attribute also applies to override implementations.

[\[attributes.codegen.track\_caller.extern\]](#r-attributes.codegen.track_caller.extern "attributes.codegen.track_caller.extern")

When applied to a function in an `extern` block the attribute must also be applied to any linked implementations, otherwise undefined behavior results. When applied to a function which is made available to an `extern` block, the declaration in the `extern` block must also have the attribute, otherwise undefined behavior results.

[\[attributes.codegen.track\_caller.behavior\]](#r-attributes.codegen.track_caller.behavior "attributes.codegen.track_caller.behavior")

### [Behavior](#behavior)

Applying the attribute to a function `f` allows code within `f` to get a hint of the [`Location`](../../core/panic/location/struct.Location.html) of the “topmost” tracked call that led to `f`’s invocation. At the point of observation, an implementation behaves as if it walks up the stack from `f`’s frame to find the nearest frame of an *unattributed* function `outer`, and it returns the [`Location`](../../core/panic/location/struct.Location.html) of the tracked call in `outer`.

```rust
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
}
```

> Note
> 
> `core` provides [`core::panic::Location::caller`](../../core/panic/location/struct.Location.html#method.caller) for observing caller locations. It wraps the [`core::intrinsics::caller_location`](../../core/intrinsics/fn.caller_location.html) intrinsic implemented by `rustc`.

> Note
> 
> Because the resulting `Location` is a hint, an implementation may halt its walk up the stack early. See [Limitations](#limitations) for important caveats.

#### [Examples](#examples)

When `f` is called directly by `calls_f`, code in `f` observes its callsite within `calls_f`:

```rust
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
fn calls_f() {
    f(); // <-- f() prints this location
}
}
```

When `f` is called by another attributed function `g` which is in turn called by `calls_g`, code in both `f` and `g` observes `g`’s callsite within `calls_g`:

```rust
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
#[track_caller]
fn g() {
    println!("{}", std::panic::Location::caller());
    f();
}

fn calls_g() {
    g(); // <-- g() prints this location twice, once itself and once from f()
}
}
```

When `g` is called by another attributed function `h` which is in turn called by `calls_h`, all code in `f`, `g`, and `h` observes `h`’s callsite within `calls_h`:

```rust
#![allow(unused)]
fn main() {
#[track_caller]
fn f() {
    println!("{}", std::panic::Location::caller());
}
#[track_caller]
fn g() {
    println!("{}", std::panic::Location::caller());
    f();
}
#[track_caller]
fn h() {
    println!("{}", std::panic::Location::caller());
    g();
}

fn calls_h() {
    h(); // <-- prints this location three times, once itself, once from g(), once from f()
}
}
```

And so on.

[\[attributes.codegen.track\_caller.limits\]](#r-attributes.codegen.track_caller.limits "attributes.codegen.track_caller.limits")

### [Limitations](#limitations)

[\[attributes.codegen.track\_caller.hint\]](#r-attributes.codegen.track_caller.hint "attributes.codegen.track_caller.hint")

This information is a hint and implementations are not required to preserve it.

[\[attributes.codegen.track\_caller.decay\]](#r-attributes.codegen.track_caller.decay "attributes.codegen.track_caller.decay")

In particular, coercing a function with `#[track_caller]` to a function pointer creates a shim which appears to observers to have been called at the attributed function’s definition site, losing actual caller information across virtual calls. A common example of this coercion is the creation of a trait object whose methods are attributed.

> Note
> 
> The aforementioned shim for function pointers is necessary because `rustc` implements `track_caller` in a codegen context by appending an implicit parameter to the function ABI, but this would be unsound for an indirect call because the parameter is not a part of the function’s type and a given function pointer type may or may not refer to a function with the attribute. The creation of a shim hides the implicit parameter from callers of the function pointer, preserving soundness.

[\[attributes.codegen.instruction\_set\]](#r-attributes.codegen.instruction_set "attributes.codegen.instruction_set")

## [The `instruction_set` attribute](#the-instruction_set-attribute)

[\[attributes.codegen.instruction\_set.intro\]](#r-attributes.codegen.instruction_set.intro "attributes.codegen.instruction_set.intro")

The *`instruction_set` [attribute](../attributes.html)* specifies the instruction set that a function will use during code generation. This allows mixing more than one instruction set in a single program.

> Example
> 
> ```rust
> #[instruction_set(arm::a32)]
> fn arm_code() {}
> 
> #[instruction_set(arm::t32)]
> fn thumb_code() {}
> ```

[\[attributes.codegen.instruction\_set.syntax\]](#r-attributes.codegen.instruction_set.syntax "attributes.codegen.instruction_set.syntax")

The `instruction_set` attribute uses the [MetaListPaths](../attributes.html#grammar-MetaListPaths) syntax to specify a single path consisting of the architecture family name and instruction set name.

[\[attributes.codegen.instruction\_set.allowed-positions\]](#r-attributes.codegen.instruction_set.allowed-positions "attributes.codegen.instruction_set.allowed-positions")

The `instruction_set` attribute may only be applied to functions with [bodies](../items/functions.html#r-items.fn.body) — [closures](../expressions/closure-expr.html#r-expr.closure), [async blocks](../expressions/block-expr.html#r-expr.block.async), [free functions](../items/functions.html#r-items.fn), [associated functions](../items/associated-items.html#r-items.associated.fn) in an [inherent impl](../items/implementations.html#r-items.impl.inherent) or [trait impl](../items/implementations.html#r-items.impl.trait), and associated functions in a [trait definition](../items/traits.html#r-items.traits) when those functions have a [default definition](../items/traits.html#r-items.traits.associated-item-decls) .

> Note
> 
> `rustc` ignores use in other positions but lints against it. This may become an error in the future.

> Note
> 
> Though the attribute can be applied to [closures](../expressions/closure-expr.html#r-expr.closure) and [async blocks](../expressions/block-expr.html#r-expr.block.async), the usefulness of this is limited as we do not yet support attributes on expressions.

[\[attributes.codegen.instruction\_set.duplicates\]](#r-attributes.codegen.instruction_set.duplicates "attributes.codegen.instruction_set.duplicates")

The `instruction_set` attribute may be used only once on a function.

[\[attributes.codegen.instruction\_set.target-limits\]](#r-attributes.codegen.instruction_set.target-limits "attributes.codegen.instruction_set.target-limits")

The `instruction_set` attribute may only be used with a target that supports the given value.

[\[attributes.codegen.instruction\_set.inline-asm\]](#r-attributes.codegen.instruction_set.inline-asm "attributes.codegen.instruction_set.inline-asm")

When the `instruction_set` attribute is used, any inline assembly in the function must use the specified instruction set instead of the target default.

[\[attributes.codegen.instruction\_set.arm\]](#r-attributes.codegen.instruction_set.arm "attributes.codegen.instruction_set.arm")

### [`instruction_set` on ARM](#instruction_set-on-arm)


...665 bytes truncated...

The tool call succeeded but the output was truncated. Full output saved to: /home/federico/.local/share/opencode/tool-output/tool_d37e159e0001aO1fCH37IdVS78
Use the Task tool to have explore agent process this file with Grep and Read (with offset/limit). Do NOT read the full file yourself - delegate to save context.
```

**Tool: webfetch**

**Input:**
```json
{
  "url": "https://std-dev-guide.rust-lang.org/policy/inline.html",
  "format": "markdown"
}
```

**Output:**
```
 When to #\[inline\] - Standard library developers Guide                      

1.  [**1.** About this Guide](../about.html)
2.  **2.** Contributing to the standard libraries
    
3.  1.  [**2.1.** Building and debugging](../development/building-and-debugging.html)
    2.  [**2.2.** Performance optimizations and benchmarking](../development/perf-benchmarking.html)
    3.  [**2.3.** Writing documentation](../development/how-to-write-documentation.html)
    4.  [**2.4.** Feature lifecycle](../development/feature-lifecycle.html)
    5.  [**2.5.** Stabilizing a feature](../development/stabilization.html)
4.  [**3.** Breaking changes](../breaking-changes/summary.html)
5.  1.  [**3.1.** New trait implementations](../breaking-changes/new-trait-impls.html)
    2.  [**3.2.** Prelude](../breaking-changes/prelude.html)
    3.  [**3.3.** Doc changes](../breaking-changes/doc-changes.html)
6.  **4.** Policies
    
7.  1.  [**4.1.** When to add #\[must\_use\]](../policy/must-use.html)
    2.  [**4.2.** Specialization](../policy/specialization.html)
    3.  [**4.3.** When to #\[inline\]](../policy/inline.html)
    4.  [**4.4.** Doc alias policy](../policy/doc-alias.html)
    5.  [**4.5.** Safety comments policy](../policy/safety-comments.html)
    6.  [**4.6.** Reviewing target-specific code](../policy/target-code.html)
    7.  [**4.7.** Why the standard library uses extension traits](../policy/extension-traits.html)
8.  **5.** Tricky situations
    
9.  1.  [**5.1.** Drop and #\[may\_dangle\]](../tricky/may-dangle.html)
    2.  [**5.2.** Generics and unsafe](../tricky/generics-and-unsafe.html)
10.  [**6.** The library team](../team/summary.html)
11.  1.  [**6.1.** Meetings](../team/meetings.html)
    2.  [**6.2.** Membership](../team/membership.html)
    3.  [**6.3.** Reviewing](../team/reviewing.html)

-   Light
-   Rust
-   Coal
-   Navy
-   Ayu

# Standard library developers Guide

[](../print.html "Print this book")[](https://github.com/rust-lang/std-dev-guide "Git repository")[](https://github.com/rust-lang/std-dev-guide/edit/master/./src/policy/inline.md "Suggest an edit")

# [When to `#[inline]`](#when-to-inline)

Inlining is a trade-off between potential execution speed, compile time and code size. There's some discussion about it in [this PR to the `hashbrown` crate](https://github.com/rust-lang/hashbrown/pull/119). From the thread:

> `#[inline]` is very different than simply just an inline hint. As I mentioned before, there's no equivalent in C++ for what `#[inline]` does. In debug mode rustc basically ignores `#[inline]`, pretending you didn't even write it. In release mode the compiler will, by default, codegen an `#[inline]` function into every single referencing codegen unit, and then it will also add `inlinehint`. This means that if you have 16 CGUs and they all reference an item, every single one is getting the entire item's implementation inlined into it.

You can add `#[inline]`:

-   To public, small, non-generic functions.

You shouldn't need `#[inline]`:

-   On methods that have any generics in scope.
-   On methods on traits that don't have a default implementation.

`#[inline]` can always be introduced later, so if you're in doubt they can just be removed.

## [What about `#[inline(always)]`?](#what-about-inlinealways)

You should just about never need `#[inline(always)]`. It may be beneficial for private helper methods that are used in a limited number of places or for trivial operators. A micro benchmark should justify the attribute.

## [For reviewers](#for-reviewers)

`#[inline]` can always be added later, so if there's any debate about whether it's appropriate feel free to defer it by removing the annotations for a start.

[](../policy/specialization.html "Previous chapter")[](../policy/doc-alias.html "Next chapter")

[](../policy/specialization.html "Previous chapter")[](../policy/doc-alias.html "Next chapter")
```

**Tool: webfetch**

**Input:**
```json
{
  "url": "https://matklad.github.io/2021/07/09/inline-in-rust.html",
  "format": "markdown"
}
```

**Output:**
```
  Inline In Rust       

[matklad](/)[About](/about.html)[Links](/links.html)[Blogroll](/blogroll.html)

# Inline In Rust

Jul 9, 2021

There’s a lot of tribal knowledge surrounding [`#[inline]`](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute) attribute in Rust. I often find myself teaching how it works, so I finally decided to write this down.

Caveat Emptor: this is what I know, not necessarily what is true. Additionally, exact semantics of `#[inline]` is not set in stone and may change in future Rust versions.

## [Why Inlining Matters?](#Why-Inlining-Matters)

Inlining is an optimizing transformation which replaces a call to a function with its body.

To give a trivial example, during compilation the compiler can transform this code:

```
fn f(w: u32) -> u32 {
    inline_me(w, 2)
}

fn inline_me(x: u32, y: u32) -> u32 {
    x * y
}
```

Into this code:

```
fn f(w: u32) -> u32 {
    w * 2
}
```

To paraphrase [A Catalogue of Optimizing Transformations](https://www.clear.rice.edu/comp512/Lectures/Papers/1971-allen-catalog.pdf) by [Frances Allen](https://en.wikipedia.org/wiki/Frances_Allen) and [John Cocke](https://en.wikipedia.org/wiki/John_Cocke):

```
There are many obvious advantages to inlining; two are:

a. There is no function call overhead whatsoever.
b. Caller and callee are optimized together. Advantage can be taken
   of particular argument values and relationships: constant arguments
   can be folded into the code, invariant instructions in the callee
   can be moved to infrequently executed areas of the caller, etc.
```

In other words, for an ahead of time compiled language inlining is the mother of all other optimizations. It gives the compiler the necessary context to apply further transformations.

## [Inlining and Separate Compilation](#Inlining-and-Separate-Compilation)

Inlining is at odds with another important idea in compilers — that of separate compilation. When compiling big programs, it is desirable to separate them into modules which can be compiled independently to:

-   Process everything in parallel.
-   Scope incremental recompilations to individual changed modules.

To achieve separate compilation, compilers expose signatures of functions, but keep function bodies invisible to other modules, preventing inlining. This fundamental tension is what makes `#[inline]` in Rust trickier than just a hint for the compiler to inline the function.

## [Inlining in Rust](#Inlining-in-Rust)

In Rust, a unit of (separate) compilation is a crate. If a function `f` is defined in a crate `A`, then all calls to `f` from within `A` can be inlined, as the compiler has full access to `f`. If, however, `f` is called from some downstream crate `B`, such calls can’t be inlined. `B` has access only to the signature of `f`, not its body.

That’s where the main usage of `#[inline]` comes from — it enables cross-crate inlining. Without `#[inline]`, even the most trivial of functions can’t be inlined across the crate boundary. The benefit is not without a cost — the compiler implements this by compiling a separate copy of the `#[inline]` function with every crate it is used in, significantly increasing compile times.

Besides `#[inline]`, there are two more exceptions to this. Generic functions are implicitly inlinable. Indeed, the compiler can only compile a generic function when it knows the specific type arguments it is instantiated with. As that is known only in the calling crate, bodies of generic functions have to be always available.

The other exception is link-time optimization. LTO opts out of separate compilation — it makes bodies of all functions available, at the cost of making compilation much slower.

## [Inlining in Practice](#Inlining-in-Practice)

Now that the underlying semantics is explained, it’s possible to infer some rule-of-thumbs for using `#[inline]`.

*First*, it’s not a good idea to apply `#[inline]` indiscriminately, as that makes compile time worse. If you don’t care about compile times, a much better solution is to set `lto = true` in Cargo profile ([docs](https://doc.rust-lang.org/cargo/reference/profiles.html#lto)).

*Second*, it usually isn’t necessary to apply `#[inline]` to private functions — within a crate, the compiler generally makes good inline decisions. There’s [a joke](https://twitter.com/ManishEarth/status/936084757212946432) that LLVM’s heuristic for when the function should be inlined is “yes”.

*Third*, when building an application, apply `#[inline]` reactively when profiling shows that a particular small function is a bottleneck. Consider using lto for releases. It might make sense to proactively `#[inline]` trivial public functions.

*Fourth*, when building libraries, proactively add `#[inline]` to small non-generic functions. Pay special attention to impls: `Deref`, `AsRef` and the like often benefit from inlining. A library can’t anticipate all usages upfront, it makes sense to not prematurely pessimize future users. Note that `#[inline]` is not transitive: if a trivial public function calls a trivial private function, you need to `#[inline]` both. See [this benchmark](https://github.com/matklad/benchmarks/tree/91171269f0a6e260a27111d07661021a89d20085/rust-inline) for details.

*Fifth*, mind generic functions. It’s not too wrong to say that generic functions are implicitly inline. As a result, they often are a cause for code bloat. Generic functions, especially in libraries, should be written to minimize unwanted inlining. To give an example from [wat](https://github.com/bytecodealliance/wasm-tools/blob/0486fb4de505b8116a0034bdde4918cd783325b9/crates/wat/src/lib.rs#L214-L222):

```
// Public, generic function.
// Will cause code bloat if not handled carefully!
pub fn parse_str(wat: impl AsRef<str>) -> Result<Vec<u8>> {
  // Immediately delegate to a non-generic function.
  _parse_str(wat.as_ref())
}

// Separate-compilation friendly private implementation.
fn _parse_str(wat: &str) -> Result<Vec<u8>> {
    ...
}
```

## [References](#References)

1.  [Language reference](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute).
2.  [Rust performance book](https://nnethercote.github.io/perf-book/inlining.html).
3.  @alexcrichton [explains inline](https://github.com/rust-lang/hashbrown/pull/119#issuecomment-537539046). Note that, in reality, the compile time costs are worse than what I described — inline functions are compiled per codegen-unit, not per crate.
4.  [More @alexcrichton](https://users.rust-lang.org/t/enable-cross-crate-inlining-without-suggesting-inlining/55004/9?u=matklad).
5.  [Even more @alexcrichton](https://internals.rust-lang.org/t/inlining-policy-for-functions-in-std/14189/10?u=matklad).

Discussion on [/r/rust](https://old.reddit.com/r/rust/comments/oh4s2j/blog_post_inline_in_rust/).

There is now a follow up post: [It’s Not Always iCache](https://matklad.github.io/2021/07/10/its-not-always-icache.html).

This post is a part of [“One Hundred Thousand Lines of Rust”](https://matklad.github.io/2021/09/05/Rust100k.html).

[Fix typo](https://github.com/matklad/matklad.github.io/edit/master/content/posts/2021-07-09-inline-in-rust.dj) [Subscribe](/feed.xml)[All posts](/)[Get in touch](mailto:aleksey.kladov+blog@gmail.com)[matklad](https://github.com/matklad)
```

---

## Assistant (Build · mimo-v2-omni-free · 10.4s)

_Thinking:_

Now I have comprehensive information about `#[inline]`. I need to synthesize this into a clear, concise explanation for the user. The user asked to "run 'explain #[inline] Rust annotation'", which I interpret as they want an explanation. I already provided a brief one, but maybe they want more detail. Let me provide a thorough explanation based on the fetched information.

Key points:

1. **What `#[inline]` does**: It's a hint to the compiler to inline a function's code at the call site, replacing the call with the function body.

2. **Three forms**:
   - `#[inline]`: Suggests inlining; compiler may ignore.
   - `#[inline(always)]`: Strongly suggests always inlining (if possible).
   - `#[inline(never)]`: Suggests never inlining.

3. **Why it matters in Rust**: Inlining is crucial for optimizations (eliminates call overhead, enables further optimizations). However, Rust uses separate compilation by crate; without `#[inline]`, functions can't be inlined across crate boundaries.

4. **When to use**:
   - For small, public, non-generic functions (especially in libraries) to allow cross-crate inlining.
   - Generic functions are implicitly inlinable (no need).
   - Private functions within a crate usually don't need it (compiler decides).
   - `#[inline(always)]` is rarely needed; only for trivial hot functions.
   - `#[inline(never)]` rarely used (maybe for large functions or debugging).

5. **Costs**: Increases compile time because each inline function is compiled in every codegen unit that uses it. Use `lto = true` for similar benefits with better whole-program optimization.

6. **Best practices**: From the Standard Library developers Guide: add `#[inline]` to public, small, non-generic functions. Avoid on generic functions or trait methods without default implementations. Don't use `#[inline(always)]` unless benchmarked.

Let me provide this explanation in a concise manner, maybe in bullet points. The user likely wants a concise answer, but since they asked to "run explain", they might want a detailed explanation. I'll give a structured explanation.

I'll also mention the note about async functions and closures: `#[inline]` applies to the generated `poll` function for async functions.

Let me craft the response.

`#[inline]` is a Rust attribute that suggests whether a function's code should be copied to the call site (inlined) instead of generating a call. It has three forms:

- `#[inline]`: Suggests inlining (compiler may ignore)
- `#[inline(always)]`: Strongly suggests always inlining (if possible)
- `#[inline(never)]`: Suggests never inlining

**Key points:**

1. **Cross-crate inlining**: The primary use is to enable inlining across crate boundaries. Without `#[inline]`, even trivial functions cannot be inlined from another crate.

2. **Trade-offs**: Inlining improves performance by eliminating call overhead and enabling further optimizations, but increases compile time because the function is compiled in every codegen unit that uses it.

3. **When to use**:
   - Small, public, non-generic functions (especially in libraries)
   - Not needed for generic functions (implicitly inlinable)
   - Not needed for private functions within a crate (compiler decides)
   - `#[inline(always)]` should be used sparingly, only for trivial hot functions
   - `#[inline(never)]` is rarely needed (large functions or debugging)

4. **Alternatives**: Using `lto = true` in Cargo.toml provides similar benefits with better whole-program optimization.

5. **Best practice**: Add `#[inline]` reactively when profiling shows a bottleneck, or proactively for small public functions in libraries. Avoid indiscriminate use.

For more details, see the [Rust Reference](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute) and [Standard Library Developers Guide](https://std-dev-guide.rust-lang.org/policy/inline.html).

---

