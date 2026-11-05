# Git Raven
<div align="center">
  <a href="https://github.com/Rodgerss111?tab=repositories">
    <picture>
      <img src="https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif" alt="Git Raven Animation" width="400">
    </picture>
  </a>
</div>
Git Raven is a Rust command-line project that turns a GitHub account's public
activity into a pixel-art animal animation. It collects an account snapshot and
renders a GIF, PNG, and browser preview based on the account's activity.

Git Raven is a dynamic, Rust-based rendering engine that generates an animated raven for your GitHub profile README. Unlike static images, this raven is directly tethered to your live GitHub activity, evolving, growing, and changing colors based on your daily coding consistency and technological stack.

The engine runs entirely via GitHub Actions, automatically fetching your contribution graph every night and rendering a fresh GIF to reflect your latest work.

## Dynamic Features & Activity Tracking

The raven acts as a living reflection of your GitHub consistency, featuring strict milestones and dynamic rendering mechanics.

### The Golden Milestone
Consistency is rewarded. The raven defaults to a charcoal-dark plumage, but will transform into the **Golden Milestone** form only when you achieve both of the following:
* **500+ Lifetime Contributions:** A baseline of established GitHub activity.
* **5-Day Continuous Streak:** You must maintain a strict, unbroken 5-day daily contribution streak. 
<div align="center">
<img width="311" height="300" alt="image" src="https://github.com/user-attachments/assets/d084f3f7-2ad3-4015-8b69-a06271d66509" />
</div>
**The Penalty:** The golden form requires relentless consistency (mathematically bound to a `0.95` streak threshold). If you miss a single 24-hour window, the GitHub API registers your streak as broken, and the raven instantly reverts to its charcoal plumage to challenge you to build it back.

### Dynamic Language Aura
When the raven is in flight, it emits an orbital, figure-8 particle aura. 
* **Charcoal Form:** The aura's color is dynamically extracted from your GitHub repository analytics. It maps specifically to your tertiary (3rd) most-used language—representing the most recent or niche technology stack you are actively adopting.
* **Golden Form:** The aura transforms into a brilliant golden glow (`[255, 245, 180]`) to match the milestone plumage.

### Flight Mechanics & Growth
* **Dynamic Scaling:** The bird physically grows in size (`scale_x` and `scale_y`) as your total lifetime contributions increase.
* **Adaptive Behaviors:** The flight path changes based on your daily activity intensity. Behaviors include *Cruise*, *Hover*, *Surface Feeding*, *Deep Rest*, and *Dart*. 
* **Aerodynamic Banking:** As the raven drifts across the 256x256 frame, it physically banks and leans into its turns, with mathematically guaranteed boundary clamping to ensure its wings and aura never clip the edges.

## How It Works

This project is fully automated and serverless.

1. **Scheduled Fetch:** A GitHub Actions workflow (`.github/workflows/raven.yml`) triggers every night at midnight UTC.
2. **Data Collection:** The workflow authenticates via a temporary `GITHUB_TOKEN` and fetches your public snapshot (repositories, languages, and contribution matrix).
3. **Rust Rendering:** The engine digests the JSON, calculates your current streak and language ratios, maps the pixels, and outputs a highly optimized `koi.gif`.
4. **Auto-Commit:** The action commits the new GIF directly to the `output/` directory of this repository.

## Embedding in Your Profile

To display the live, self-updating raven on your main GitHub profile, add this HTML block to your special profile `README.md` (e.g., `Rodgerss111/Rodgerss111`):

```html
<div align="center">
  <a href="[https://github.com/Rodgerss111/raven](https://github.com/Rodgerss111/raven)">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="[https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif](https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif)">
      <source media="(prefers-color-scheme: light)" srcset="[https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif](https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif)">
      <img src="[https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif](https://raw.githubusercontent.com/Rodgerss111/raven/main/output/koi.gif)" alt="Git Raven Animation" width="400">
    </picture>
  </a>
</div>
## Usage

The project requires Rust and an authenticated
[GitHub CLI](https://cli.github.com/) installation.

```sh
cargo run -- collect USERNAME --output snapshot.json
cargo run -- render snapshot.json --output-dir output
```

The renderer supports these body styles:

```text
white, black, asagi, tea, gold, platinum
```

For example:

```sh
cargo run -- render snapshot.json --output-dir output --body black
```

Generated files include:

- `koi.gif` - a looping animation
- `koi.png` - a still image
- `preview.html` - a light and dark background preview

## Inspiration and Creative Direction

This repository is a reference to **Anthony Yalong**'s original project. I
modified and refined the idea in my own way to create a new animal that is
different from the original koi fish: a raven.

I chose the raven because it represents my creativity in everything I do. To
me, the raven represents curiosity, intelligence, and inspiration. It is also a
trickster figure connected to creation and shifting perspectives.

## License

This project is released under the MIT License.
