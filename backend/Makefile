.PHONY: print_schema
print_schema:
	cd crop_infra && make print_schema


.PHONY: test
test:
	bash tests/run-all.sh

.PHONY: print_sample_credential
print_sample_credential:
	cargo run --example print_sample_credential
