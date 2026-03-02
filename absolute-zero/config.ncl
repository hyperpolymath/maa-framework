# Absolute Zero - Nickel Configuration
# Central configuration for the CNO formal verification project

{
  project = {
    name = "Absolute Zero",
    version = "0.1.0",
    description = "Formal verification of Certified Null Operations",
    author = "Jonathan D. A. Jewell",
    license = ["AGPL-3.0", "Palimpsest-0.5"],
    repository = {
      gitlab = "https://gitlab.com/maa-framework/6-the-foundation/absolute-zero",
      github = "https://github.com/Hyperpolymath/absolute-zero",
    },
  },

  provers = {
    coq = {
      version = "8.19",
      enabled = true,
      source_dirs = ["proofs/coq/common", "proofs/coq/physics", "proofs/coq/quantum", "proofs/coq/category", "proofs/coq/lambda", "proofs/coq/malbolge"],
      build_command = "coqc",
      verify_command = "coqc",
    },
    lean4 = {
      version = "4.0",
      enabled = true,
      source_dir = "proofs/lean4",
      build_command = "lake build",
      verify_command = "lake build",
    },
    z3 = {
      version = "4.13",
      enabled = true,
      source_file = "proofs/z3/cno_properties.smt2",
      verify_command = "z3",
    },
    agda = {
      version = "2.6",
      enabled = true,
      source_dir = "proofs/agda",
      build_command = "agda",
    },
    isabelle = {
      version = "2023",
      enabled = true,
      source_dir = "proofs/isabelle",
      build_command = "isabelle build -D .",
    },
  },

  interpreters = {
    malbolge_rescript = {
      language = "ReScript",
      source = "interpreters/rescript/malbolgeInterpreter.res",
      build = "npx rescript build",
    },
    brainfuck_python = {
      language = "Python",
      source = "interpreters/brainfuck/brainfuck.py",
      interpreter = "python3",
    },
    whitespace_python = {
      language = "Python",
      source = "interpreters/whitespace/whitespace.py",
      interpreter = "python3",
    },
  },

  build = {
    tools = {
      just = { required = true, cookbook = "justfile-cookbook.adoc" },
      deno = { required = true, config = "deno.json" },
      lake = { required = true, config = "proofs/lean4/lakefile.lean" },
      rescript = { required = false, version = "11.1" },
    },
    targets = ["coq", "lean4", "z3", "agda", "isabelle", "rescript", "deno"],
    output_dir = "dist",
    wasm_enabled = true,
  },

  documentation = {
    format = "asciidoc",
    main_files = ["README.adoc", "CLAUDE.adoc", "COOKBOOK.adoc"],
    api_docs_dir = "docs/api",
    theorem_docs_dir = "docs/theorems",
    generator = "asciidoctor",
  },

  testing = {
    unit_tests = "tests/unit",
    integration_tests = "tests/integration",
    proof_tests = "tests/proofs",
    property_based = {
      echidna = {
        enabled = true,
        config = "echidna.yml",
        contracts_dir = "tests/contracts",
      },
      quickcheck = {
        enabled = false,
      },
      hypothesis = {
        enabled = false,
      },
    },
  },

  examples = {
    esoteric = ["malbolge", "brainfuck", "whitespace"],
    mainstream = ["ada", "assembly", "c", "cpp", "csharp", "clojure", "cobol", "elixir", "erlang", "fortran", "fsharp", "go", "haskell", "java", "javascript", "kotlin", "lisp", "ocaml", "perl", "php", "prolog", "python", "ruby", "rust", "scala", "scheme", "swift"],
    special_ops = {
      cpu = ["assembly"],
      database = ["sql"],
      gpu = ["cuda", "opencl", "glsl"],
      hardware = ["verilog", "vhdl"],
      network = ["http", "tcp", "udp"],
      os = ["syscall", "signal", "futex", "ioctl"],
      quantum = ["qasm"],
    },
  },

  container = {
    engine = "podman",
    fallback = "docker",
    file = "Containerfile",
    image_name = "absolute-zero",
    tag = "latest",
  },

  ci_cd = {
    gitlab_ci = {
      enabled = true,
      file = ".gitlab-ci.yml",
    },
    github_actions = {
      enabled = false,
    },
  },

  compliance = {
    tech_stack = {
      required = {
        rescript = "For type-safe interpreters",
        deno = "For runtime and scripting",
        nickel = "For configuration",
        asciidoc = "For documentation",
        just = "For build automation",
        oil_shell = "For advanced shell scripting",
        wasm = "For performance-critical code",
      },
      forbidden = {
        typescript = "Use Deno with JSDoc types instead",
        npm = "Use Deno's built-in package management",
        markdown = "Use AsciiDoc (.adoc) except for SECURITY.md",
      },
    },
  },
}
