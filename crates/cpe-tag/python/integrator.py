from cpe_tag.cpe import convert_quasi_cpe_to_regex
from cpe_tag.serializers import serialize_package_json


def run(payload: list) -> str:
    if isinstance(payload, list):
        cpe_patterns = handle_list(payload)
    else:
        raise TypeError(f"expected list got {type(payload)}")

    return "|".join(cpe_patterns)


def handle_list(packages: list) -> list:
    cpe_patterns = []
    for package in packages:
        cpe_patterns = cpe_patterns + handle_dict(package)
    return cpe_patterns


def handle_dict(package: dict) -> list:
    serialized = serialize_package_json(package)
    quasi_cpes = list(map(lambda x: x.get("quasi_cpe"), serialized["versions"]))
    quasi_cpes = list(filter(lambda x: x is not None, quasi_cpes))
    return [convert_quasi_cpe_to_regex(qc) for qc in quasi_cpes]
