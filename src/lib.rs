use zed_extension_api::{
    self as zed,
    process::Command,
    SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

struct SlashDevtools;

impl zed::Extension for SlashDevtools {
    fn new() -> Self {
        SlashDevtools
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "todo" | "deps" | "stack" | "recent" => Ok(vec![]),
            cmd => Err(format!("unknown command: \"{cmd}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let worktree = worktree.ok_or("no worktree available")?;
        let root = worktree.root_path();

        match command.name.as_str() {
            "todo" => run_todo(&root),
            "deps" => run_deps(worktree),
            "stack" => run_stack(worktree),
            "recent" => run_recent(&root),
            cmd => Err(format!("unknown command: \"{cmd}\"")),
        }
    }
}

fn run_todo(root: &str) -> Result<SlashCommandOutput, String> {
    let output = Command::new("grep")
        .args([
            "-rn",
            "--include=*.ts",
            "--include=*.tsx",
            "--include=*.js",
            "--include=*.jsx",
            "--include=*.rs",
            "--include=*.py",
            "--include=*.go",
            "--include=*.java",
            "--include=*.c",
            "--include=*.cpp",
            "--include=*.cs",
            "--include=*.rb",
            "--include=*.lua",
            "--include=*.svelte",
            "--include=*.vue",
            "--include=*.css",
            "--include=*.scss",
            "-E",
            r"(TODO|FIXME|HACK|BUG|WARN|NOTE)\b",
            root,
        ])
        .output()
        .map_err(|e| format!("failed to run grep: {e}"))?;

    let text = if output.stdout.is_empty() {
        "No TODO/FIXME/HACK/BUG comments found.".to_string()
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Strip the root path prefix for cleaner output
        let lines: Vec<&str> = stdout.lines().collect();
        let cleaned: Vec<String> = lines
            .iter()
            .map(|line| line.replacen(root, ".", 1))
            .collect();
        format!(
            "Found {} TODO/FIXME/HACK/BUG comments:\n\n{}",
            cleaned.len(),
            cleaned.join("\n")
        )
    };

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: "TODOs".to_string(),
        }],
        text,
    })
}

fn run_deps(worktree: &Worktree) -> Result<SlashCommandOutput, String> {
    let mut sections = Vec::new();
    let mut text = String::new();

    // Try package.json
    if let Ok(content) = worktree.read_text_file("package.json") {
        let label = "package.json dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## package.json\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    // Try Cargo.toml
    if let Ok(content) = worktree.read_text_file("Cargo.toml") {
        let label = "Cargo.toml dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## Cargo.toml\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    // Try requirements.txt
    if let Ok(content) = worktree.read_text_file("requirements.txt") {
        let label = "requirements.txt dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## requirements.txt\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    // Try pyproject.toml
    if let Ok(content) = worktree.read_text_file("pyproject.toml") {
        let label = "pyproject.toml dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## pyproject.toml\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    // Try go.mod
    if let Ok(content) = worktree.read_text_file("go.mod") {
        let label = "go.mod dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## go.mod\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    // Try Gemfile
    if let Ok(content) = worktree.read_text_file("Gemfile") {
        let label = "Gemfile dependencies".to_string();
        let start = text.len();
        text.push_str(&format!("## Gemfile\n\n{content}\n\n"));
        sections.push(SlashCommandOutputSection {
            range: (start..text.len()).into(),
            label,
        });
    }

    if text.is_empty() {
        text = "No dependency files found (checked package.json, Cargo.toml, requirements.txt, pyproject.toml, go.mod, Gemfile).".to_string();
        sections.push(SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: "Dependencies".to_string(),
        });
    }

    Ok(SlashCommandOutput { sections, text })
}

fn run_stack(worktree: &Worktree) -> Result<SlashCommandOutput, String> {
    let mut detected = Vec::new();

    // Package managers
    if worktree.read_text_file("package-lock.json").is_ok() {
        detected.push("npm (package-lock.json)");
    }
    if worktree.read_text_file("yarn.lock").is_ok() {
        detected.push("Yarn (yarn.lock)");
    }
    if worktree.read_text_file("pnpm-lock.yaml").is_ok() {
        detected.push("pnpm (pnpm-lock.yaml)");
    }
    if worktree.read_text_file("bun.lockb").is_ok() || worktree.read_text_file("bun.lock").is_ok()
    {
        detected.push("Bun (bun.lock)");
    }

    // Languages & runtimes
    if worktree.read_text_file("tsconfig.json").is_ok() {
        detected.push("TypeScript (tsconfig.json)");
    }
    if worktree.read_text_file("Cargo.toml").is_ok() {
        detected.push("Rust (Cargo.toml)");
    }
    if worktree.read_text_file("go.mod").is_ok() {
        detected.push("Go (go.mod)");
    }
    if worktree.read_text_file("requirements.txt").is_ok()
        || worktree.read_text_file("pyproject.toml").is_ok()
    {
        detected.push("Python");
    }
    if worktree.read_text_file("Gemfile").is_ok() {
        detected.push("Ruby (Gemfile)");
    }

    // Frameworks (check package.json content)
    if let Ok(pkg) = worktree.read_text_file("package.json") {
        if pkg.contains("\"next\"") {
            detected.push("Next.js");
        }
        if pkg.contains("\"react\"") && !pkg.contains("\"next\"") {
            detected.push("React");
        }
        if pkg.contains("\"vue\"") {
            detected.push("Vue");
        }
        if pkg.contains("\"svelte\"") || pkg.contains("\"@sveltejs") {
            detected.push("Svelte");
        }
        if pkg.contains("\"angular\"") || pkg.contains("\"@angular") {
            detected.push("Angular");
        }
        if pkg.contains("\"express\"") {
            detected.push("Express.js");
        }
        if pkg.contains("\"fastify\"") {
            detected.push("Fastify");
        }
        if pkg.contains("\"hono\"") {
            detected.push("Hono");
        }
        if pkg.contains("\"astro\"") {
            detected.push("Astro");
        }
        if pkg.contains("\"remix\"") || pkg.contains("\"@remix-run") {
            detected.push("Remix");
        }
        if pkg.contains("\"tailwindcss\"") {
            detected.push("Tailwind CSS");
        }
        if pkg.contains("\"prisma\"") || pkg.contains("\"@prisma") {
            detected.push("Prisma");
        }
        if pkg.contains("\"drizzle-orm\"") {
            detected.push("Drizzle ORM");
        }
        if pkg.contains("\"vite\"") {
            detected.push("Vite");
        }
        if pkg.contains("\"vitest\"") {
            detected.push("Vitest");
        }
        if pkg.contains("\"jest\"") {
            detected.push("Jest");
        }
        if pkg.contains("\"playwright\"") || pkg.contains("\"@playwright") {
            detected.push("Playwright");
        }
        if pkg.contains("\"react-native\"") {
            detected.push("React Native");
        }
        if pkg.contains("\"expo\"") {
            detected.push("Expo");
        }
        if pkg.contains("\"electron\"") {
            detected.push("Electron");
        }
        if pkg.contains("\"tauri\"") || pkg.contains("\"@tauri") {
            detected.push("Tauri");
        }
    }

    // CI/CD & Infrastructure
    if worktree.read_text_file(".github/workflows").is_ok()
        || worktree.read_text_file(".github/workflows/ci.yml").is_ok()
    {
        detected.push("GitHub Actions");
    }
    if worktree.read_text_file("Dockerfile").is_ok() {
        detected.push("Docker");
    }
    if worktree.read_text_file("docker-compose.yml").is_ok()
        || worktree.read_text_file("docker-compose.yaml").is_ok()
    {
        detected.push("Docker Compose");
    }
    if worktree.read_text_file("vercel.json").is_ok() {
        detected.push("Vercel");
    }
    if worktree.read_text_file("netlify.toml").is_ok() {
        detected.push("Netlify");
    }

    // Config & tooling
    if worktree.read_text_file(".eslintrc.json").is_ok()
        || worktree.read_text_file(".eslintrc.js").is_ok()
        || worktree.read_text_file("eslint.config.js").is_ok()
        || worktree.read_text_file("eslint.config.mjs").is_ok()
    {
        detected.push("ESLint");
    }
    if worktree.read_text_file(".prettierrc").is_ok()
        || worktree.read_text_file(".prettierrc.json").is_ok()
        || worktree.read_text_file("prettier.config.js").is_ok()
    {
        detected.push("Prettier");
    }
    if worktree.read_text_file("biome.json").is_ok() {
        detected.push("Biome");
    }

    let text = if detected.is_empty() {
        "Could not detect any known technologies in this project.".to_string()
    } else {
        format!(
            "Detected {} technologies:\n\n{}",
            detected.len(),
            detected
                .iter()
                .map(|t| format!("- {t}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: "Tech Stack".to_string(),
        }],
        text,
    })
}

fn run_recent(root: &str) -> Result<SlashCommandOutput, String> {
    let output = Command::new("git")
        .args([
            "-C",
            root,
            "log",
            "--oneline",
            "--no-decorate",
            "-20",
            "--format=%h %s (%ar)",
        ])
        .output()
        .map_err(|e| format!("failed to run git log: {e}"))?;

    let log = String::from_utf8_lossy(&output.stdout);

    if log.trim().is_empty() {
        return Err("no git history found".to_string());
    }

    // Get the diff stat for the last 5 commits
    let diff_output = Command::new("git")
        .args(["-C", root, "diff", "--stat", "HEAD~5..HEAD"])
        .output()
        .map_err(|e| format!("failed to run git diff: {e}"))?;

    let diff_stat = String::from_utf8_lossy(&diff_output.stdout);

    let text = format!(
        "## Recent commits (last 20)\n\n{}\n\n## Changes in last 5 commits\n\n{}",
        log.trim(),
        if diff_stat.trim().is_empty() {
            "No diff stats available.".to_string()
        } else {
            diff_stat.trim().to_string()
        }
    );

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: "Recent Git Activity".to_string(),
        }],
        text,
    })
}

zed::register_extension!(SlashDevtools);
