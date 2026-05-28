# ChemiCLI

ChemiCLI is a command-line interface for querying information about the periodic table of elements. It provides detailed properties for individual elements and information about various chemical series.

## Features

- **Element Querying**: Retrieve atomic number, weight, energy levels, electronegativity, and more by atomic symbol.
- **Metal Series**: Explore subgroups of metals including Alkali, Alkaline Earth, Lanthanoids, Actinoids, Transition, and Post-transition metals.
- **Local Data**: Uses a local JSON-based periodic table for fast, offline access.

## Installation

To install ChemiCLI, you need to have [Rust and Cargo](https://rustup.rs/) installed.

1. Clone the repository:
   ```sh
   git clone https://github.com/overoxidize/chemicli.git
   cd chemicli
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

The binary will be available at `target/release/chemicli`.

## Usage

```sh
chemicli <COMMAND>
```

### Element Command

Query information about a specific element using its atomic symbol.

```sh
chemicli element <ATOMIC_SYMBOL> [OPTIONS]
```

**Options:**
- `-n, --atomic-number`: Include the atomic number.
- `-w, --atomic-weight`: Include the atomic weight.
- `-l, --energy-levels`: Include the energy levels.
- `-e, --electronegativity`: Include the electronegativity.
- `-s, --series`: Include the series.
- `-d, --date-discovered`: Include the discovery date.
- `-D, --discoverer`: Include the discoverer.
- `-g, --group`: Include the group.
- `-p, --period`: Include the period.

### Metals Command

Query information about specific metal series. Alias: `mtl`.

```sh
chemicli metals <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**
- `alkali` (alias: `alki`)
- `alkaline` (alias: `alkn`)
- `lanthanoids` (aliases: `lan`, `lanthanides`)
- `actinoids` (aliases: `act`, `actinides`)
- `transition` (alias: `trans`)
- `post-transition` (alias: `post`)

**Options:**
- `-m, --members`: List all members of the series.
- `-n, --number`: Show the total number of members in the series.

## Examples

- Get the atomic number of Oxygen:
  ```sh
  chemicli element O -n
  ```

- List all Alkali metals:
  ```sh
  chemicli mtl alkali --members
  ```

- Get the atomic weight and period of Iron:
  ```sh
  chemicli element Fe -w -p
  ```

## Data Source

ChemiCLI uses a local JSON file located at `data/periodic_table.json`. The data is based on information from [PTable](https://ptable.com/), though some values may vary slightly from standard IUPAC values.

## Roadmap

- [x] Implement `metals` (`mtl`) command with subgroup support.
- [ ] Implement `metalloids` command.
- [ ] Implement `nonmetals` command.
- [ ] Add support for more detailed property comparisons.
- [ ] Improve data precision against latest scientific standards.
