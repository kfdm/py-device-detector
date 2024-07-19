VENV_DIR := .venv
PYTHON_BIN := $(VENV_DIR)/bin/python
PIP_BIN := $(VENV_DIR)/bin/pip
MATURIN := $(VENV_DIR)/bin/maturin

SYSTEM_PYTHON ?= python3.11


## Default
develop: $(MATURIN)
	$(MATURIN) develop
	$(PYTHON_BIN) shell.py


$(VENV_DIR):
	$(SYSTEM_PYTHON) -m venv $(VENV_DIR)


$(PIP_BIN): $(VENV_DIR)

$(MATURIN): $(PIP_BIN)
	$(PIP_BIN) install maturin
