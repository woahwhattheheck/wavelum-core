# Coverage target using cargo-llvm-cov
coverage:
	cargo install cargo-llvm-cov 2>/dev/null || true
	cargo llvm-cov --workspace --lcov --output-path lcov.info
	cargo llvm-cov --workspace --summary-only
	@echo "Coverage report: lcov.info"

coverage-html:
	cargo llvm-cov --workspace --html --output-dir coverage/
	@echo "HTML report: coverage/index.html"

coverage-clean:
	rm -f lcov.info
	rm -rf coverage/
