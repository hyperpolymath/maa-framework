;; SPDX-License-Identifier: MPL-2.0
;; Development manifest for maa-framework. Bare `guix shell` picks this up
;; automatically from the repo root (it searches cwd and parents for
;; manifest.scm — see "Invoking guix shell" in the manual) and provides the
;; toolchain for `just check` on the aletheia crate.
;;
;; Specs verified 2026-09-21: `rust` (unversioned alias in
;; gnu/packages/rust.scm — confirmed via Software Heritage; includes cargo),
;; `just` (gnu/packages/rust-apps.scm, 1.43.0), `git` (used in the manual's
;; own manifest example). No Guix daemon was available where this was
;; written, so it is reviewed, not executed — a bad spec fails loudly at
;; `guix shell` time; fix the spec, not the shell invocation.
(specifications->manifest
 (list "rust" "just" "git"))
