# rust-template
<div align="center">
  <!-- GitHub Profile Shield with Logo -->
  <a href="https://marcoramos17.github.io/">
    <img src="https://img.shields.io/badge/GitHub-Profile-181717?logo=github" alt="{GitHub} Profile"></a>

  <!-- Rustdoc Pages Shield with Logo -->
  <a href="https://github.com/marcoramos17">
    <img src="https://img.shields.io/badge/docs-available-brightgreen" alt="{RustDoc} Pages"></a>
</div>



# Affiliation - Project (Template)

This repository serves as a centralized template for managing Rust programming language projects efficiently. It features optional GitHub Project support (for kanban/task tracking), flexible branching, and — most notably — **automated generation and deployment of Rust documentation via GitHub Actions and GitHub Pages**.

---

## Document Information

### Affiliation:
*Example University*

### Project:
*Module/Project Name*

### Title:
*Document Title Here*

### Author(s):
*Marco Ramos - 10415201*

### Description:
A reusable and well-structured template tailored for Rust projects. Automatically builds and hosts documentation using `cargo doc` and GitHub Pages.

---

## Key Features

✅ Clean and modular Rust project structure  
✅ Automated documentation with `cargo doc`  
✅ GitHub Actions integration for generating and publishing docs  
✅ Hosted docs accessible via GitHub Pages  
✅ Suggested branching workflow for project development  
✅ GitHub Project Boards support (optional)

---

## 📄 Rust Documentation Hosting

This template includes a GitHub Actions workflow that:

1. Runs `cargo doc --no-deps` to generate documentation for the current codebase.
2. Deploys the generated documentation to the `gh-pages` branch.
3. Hosts it using GitHub Pages at:
  https://marcoramos17.github.io/<your-repo-name>/


> 🛠️ _Note: If docs are not building or the page isn't updating, check the status of your GitHub Actions and make sure your repository settings allow GitHub Pages from the `gh-pages` branch._

---

## Workflow Guide

### **Working with the Main Branch**
The `main` branch:
- Repository main branch

#### Key Actions:
- **Update the repository (`main` branch):**
  ```bash
  git add .
  git commit -m "message"
  git push
  ```

---

### **Creating a New Branch**
To start working on a new branch:

1. **Create a new branch OR switch to an existing branch:**
   ```bash
   git checkout -b branch-name
   #OR
   git checkout branch-name
   ```
   - Branch naming format: `-- no format specified --`. Ex: `-- branch_name --`

2. **Push the branch to the remote repository (optional):**
   ```bash
   git push
   ```

---

### **Keeping The Other Branches Updated**
If updates are made to the `main` branch, pull those changes into other branches:

1. **Switch to another document branch:**
   ```bash
   git checkout branch-name
   ```

2. **Pull changes from `main`:**
   ```bash
   git pull origin main
   ```

3. **Resolve any merge conflicts (if applicable):**
   Open conflicting files, resolve issues manually, and mark them resolved:
   ```bash
   git add resolved-file
   git commit
   ```

---

### *Recommended File Structure*
```
├── src/               # Main source files  
├── tests/             # Unit and integration tests  
├── target/            # Compiled artifacts (ignored by Git)  
├── .github/workflows/ # GitHub Actions for Rustdoc  
├── docs/              # Optional manual documentation  
├── Cargo.toml         # Project metadata and dependencies  
├── README.md  
└── LICENSE  
```

---

### **General Tips**
- Regularly pull updates from `main` to keep branches up-to-date with the latest template changes.
- Use descriptive branch names (e.g., `COV_6006CEM_Train-Accidents`).
- Write clear commit messages for better collaboration and tracking.

---


This README serves as an introduction to maintain an organized and efficient workflow for managing Computer Science Projects.
