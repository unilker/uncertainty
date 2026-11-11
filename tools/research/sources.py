"""Primary source of a library project (its first source) grouped into documents and kinds of document, and the
basis of its reference values, read from the reference statement of the project (D110). Used by article_stats.py
and article_supplementary.py."""
import re

# (pattern on the first source's citation, document, kind)
DOCS = [
    (r"^JCGM 100", "JCGM 100 (GUM)", "guidance"), (r"^JCGM 101", "JCGM 101", "guidance"),
    (r"^JCGM 102", "JCGM 102", "guidance"), (r"^JCGM 106", "JCGM 106", "guidance"),
    (r"^JCGM GUM-5", "JCGM GUM-5", "guidance"),
    (r"^EURACHEM/CITAC Guide CG 4", "Eurachem/CITAC QUAM", "guidance"),
    (r"compliance assessment", "Eurachem/CITAC compliance guide", "guidance"),
    (r"UfS", "Eurachem UfS guide", "guidance"),
    (r"^Eurachem/CITAC Guide", "Eurachem/CITAC MUPR guide", "guidance"),
    (r"^Nordtest TR 537", "Nordtest TR 537", "guidance"), (r"^Nordtest TR 569", "Nordtest TR 569", "guidance"),
    (r"^EA-4/02", "EA-4/02", "guidance"), (r"^UKAS M3003", "UKAS M3003", "guidance"),
    (r"Calibration Guide No. 18", "EURAMET cg-18", "guidance"), (r"Calibration Guide No. 19", "EURAMET cg-19", "guidance"),
    (r"^ISO 13528", "ISO 13528", "guidance"), (r"^ISO 5725-2", "ISO 5725-2", "guidance"),
    (r"^ISO 5725-3", "ISO 5725-3", "guidance"), (r"^ISO 21748", "ISO 21748", "guidance"),
    (r"^ISO 6143", "ISO 6143", "guidance"),
    (r"^NIST Technical Note 1900", "NIST TN 1900", "guidance"), (r"^T\. Lafarge", "NIST Uncertainty Machine manual", "guidance"),
    (r"^NIST Consensus Builder", "NIST Consensus Builder manual", "guidance"),
    (r"^Possolo et al\.", "IUPAC brief guide (2024)", "guidance"), (r"EMUE", "EMPIR EMUE compendium", "guidance"),
    (r"^BIPM", "BIPM ISRD reports", "reference data"),
    (r"^C\. Palma", "Palma et al. 2019", "paper or book"), (r"Bettencourt da Silva", "Bettencourt da Silva 2026", "paper or book"),
    (r"^Koepke", "Koepke et al. 2017", "paper or book"), (r"^Merkatas", "Merkatas et al. 2019", "paper or book"),
    (r"^Possolo & Bodnar", "Possolo & Bodnar 2018", "paper or book"), (r"^Possolo & Meija", "Possolo & Meija 2022", "paper or book"),
    (r"procedure|Prosedür", "unpublished procedure of a metrology institute", "unpublished"),
]
KIND_TEXT = {"guidance": "guidance document or standard", "reference data": "BIPM reference data report",
             "paper or book": "journal paper or book", "unpublished": "unpublished procedure"}


def primary(project):
    doc = project["metadata"]["sources"][0]["document"]
    for pat, name, kind in DOCS:
        if re.search(pat, doc):
            if name == "BIPM ISRD reports":
                m = re.search(r"ISRD-0\d", doc)
                name = f"BIPM {m.group(0)}" if m else name
            return name, kind
    raise ValueError(f"unclassified source: {doc}")


def reference_basis(project):
    r = project["expected"]["reference"]["en"]
    if re.search(r"recomput|independent|numpy|scipy|script|tools/research|exact|grid|port of", r, re.I):
        return "partly or wholly an independent recomputation"
    if re.search(r"monte carlo|mcmc|draws|trials|bootstrap", r, re.I):
        return "printed values, some of them Monte Carlo or MCMC estimates"
    return "printed values"
