import os
import subprocess
import pytest

@pytest.fixture
def plugin_path():
    return os.environ["PYLLVM_PLUGIN_PATH"]


def test_replace_return(tmp_path, plugin_path):
    src = tmp_path / 'main.cpp'
    with src.open('w') as f:
        f.write("int main() { return 0; }\n")
    ll = tmp_path/'main.ll'
    subprocess.check_call(['clang-18', '-O3', src, '-S', '-emit-llvm', '-o', ll])
    assert 'ret i32 0' in ll.read_text()

    outfile = tmp_path/'out.ll'
    subprocess.check_call([
        'opt-18',
        '--load-pass-plugin',
        plugin_path,
        '--passes=pyllvmpass[replace_return]',
        ll,
        '-S',
        '-o',
        outfile
    ])
    assert 'ret i32 0' not in outfile.read_text()
    assert 'ret i32 1' in outfile.read_text()


def test_invalid(tmp_path, plugin_path):
    src = tmp_path / 'main.cpp'
    with src.open('w') as f:
        f.write("int main() { return 0; }\n")
    ll = tmp_path/'main.ll'
    subprocess.check_call(['clang-18', '-O3', src, '-S', '-emit-llvm', '-o', ll])
    assert 'ret i32 0' in ll.read_text()

    outfile = tmp_path/'out.ll'
    with pytest.raises(subprocess.CalledProcessError):
        subprocess.check_call([
            'opt-18',
            '--load-pass-plugin',
            plugin_path,
            '--passes=pyllvmpass[invalid]',
            ll,
            '-S',
            '-o',
            outfile
        ], stderr=subprocess.DEVNULL)
