#!/usr/bin/python3

import cddlcat
import cbor2

def test_cbor_dicts_arrays():
    cddl_spec = '''person = {
      age: int,
      name: tstr,
      employer: tstr,
    }

    person_array = [
      xage: int,
      xname: tstr,
      xemployer: tstr,
    ]'''

    person2_dict = {
        'age': 70,
        'name': 'Bill Gates',
        'employer': 'Microsoft',
    }

    person2_array = [
        70,
        'Bill Gates',
        'Microsoft',
    ]

    person2_dict_cbor = cbor2.dumps(person2_dict)
    person2_array_cbor = cbor2.dumps(person2_array)

    #print('dict after round-trip:', cbor2.loads(person2_dict_cbor))
    #print('array after round-trip:', cbor2.loads(person2_array_cbor))

    # CBOR dict vs CDDL dict
    assert cddlcat.validate_cbor_bytes(cddl_spec, 'person', person2_dict_cbor)
    # CBOR array vs CDDL dict
    assert not cddlcat.validate_cbor_bytes(cddl_spec, 'person', person2_array_cbor)

    # CBOR dict vs CDDL array
    assert not cddlcat.validate_cbor_bytes(cddl_spec, 'person_array', person2_dict_cbor)
    # CBOR dict vs CDDL array
    assert cddlcat.validate_cbor_bytes(cddl_spec, 'person_array', person2_array_cbor)

def test_bad_arrays():
    cddl_spec = '''person = {
      age: int,
      name: tstr,
      employer: tstr,
    }

    person_array = [
      xage: int,
      xname: tstr,
      xemployer: tstr,
    ]'''

    bad_arrays = [
        [123, 'foo','bar', 456],
        [123, 'foo'],
        [123],
        [],
        ['abc'],
        ['abc', 123],
        [123, 'foo', 456],
        [123, ['foo', 'bar']],
        [123, ['foo'], 'bar'],
    ]

    for index, bad_array in enumerate(bad_arrays):
        bad_cbor = cbor2.dumps(bad_array)
        assert not cddlcat.validate_cbor_bytes(cddl_spec, 'person_array', bad_cbor)

test_cbor_dicts_arrays()
test_bad_arrays()
print("done!")
