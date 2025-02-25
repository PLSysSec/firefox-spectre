import json
import pathlib
import pytest
from mozperftest.tests.support import temp_dir
from mozperftest.metrics.notebook.perftestetl import PerftestETL
from mozperftest.metrics.notebook.perftestnotebook import PerftestNotebook


@pytest.fixture(scope="session", autouse=True)
def data():
    data_1 = {
        "browserScripts": [
            {"timings": {"firstPaint": 101}},
            {"timings": {"firstPaint": 102}},
            {"timings": {"firstPaint": 103}},
        ],
    }

    data_2 = {
        "browserScripts": [
            {"timings": {"firstPaint": 201}},
            {"timings": {"firstPaint": 202}},
            {"timings": {"firstPaint": 203}},
        ],
    }

    data_3 = {
        "browserScripts": [
            {"timings": {"firstPaint": 301}},
            {"timings": {"firstPaint": 302}},
            {"timings": {"firstPaint": 303}},
        ],
    }

    yield {"data_1": data_1, "data_2": data_2, "data_3": data_3}


@pytest.fixture(scope="session", autouse=True)
def standarized_data():
    return {
        "browsertime": [
            {
                "data": [
                    {"value": 1, "xaxis": 1, "file": "file_1"},
                    {"value": 2, "xaxis": 2, "file": "file_2"},
                ],
                "name": "name",
                "subtest": "subtest",
            }
        ]
    }


@pytest.fixture(scope="session", autouse=True)
def files(data):
    # Create a temporary directory.
    with temp_dir() as td:
        tmp_path = pathlib.Path(td)

    dirs = {
        "resources": tmp_path / "resources",
        "output": tmp_path / "output",
    }

    for d in dirs.values():
        d.mkdir(parents=True, exist_ok=True)

    # Create temporary data files for tests.
    def _create_temp_files(path, data):
        path.touch(exist_ok=True)
        path.write_text(data)
        return path.resolve().as_posix()

    resources = {}
    json_1 = dirs["resources"] / "file_1.json"
    resources["file_1"] = _create_temp_files(json_1, json.dumps(data["data_1"]))

    json_2 = dirs["resources"] / "file_2.json"
    resources["file_2"] = _create_temp_files(json_2, json.dumps(data["data_2"]))

    txt_3 = dirs["resources"] / "file_3.txt"
    resources["file_3"] = _create_temp_files(txt_3, str(data["data_3"]))

    output = dirs["output"] / "output.json"

    yield resources, dirs, output.resolve().as_posix()


@pytest.fixture(scope="session", autouse=True)
def ptetls(files):
    resources, dirs, output = files
    config = {"output": output}
    file_group_list = {"group_1": list(resources.values())}
    file_group_str = {"group_1": dirs["resources"].resolve().as_posix()}

    yield {
        "ptetl_list": PerftestETL(file_group_list, config, sort_files=True),
        "ptetl_str": PerftestETL(file_group_str, config, sort_files=True),
    }


@pytest.fixture(scope="session", autouse=True)
def ptnb(standarized_data):
    return PerftestNotebook(standarized_data)
