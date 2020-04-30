#!/usr/bin/python3

import cddlcat
import cbor2
import json
import unittest

class TestCddlcat(unittest.TestCase):
    def test_parsing(self):
        cddl = '''some_literal = 123.0
        person = {
          age: int,
          name: tstr,
          employer: tstr,
        }'''
        ivt = cddlcat.flatten_from_str(cddl)
        for name, rule_etc in ivt.items():
            #print('{} -> {}'.format(name, rule_etc))
            assert type(name) is str
            rule, string = rule_etc
            assert type(rule) is cddlcat.IVTNode
            assert type(string) is str


        lit_node, orig_txt = ivt['some_literal']
        assert(orig_txt == 'some_literal = 123.0')
        assert(lit_node.kind() == 'Literal')
        assert(type(lit_node.value()) == float)

        person_node, _ = ivt['person']
        assert(person_node.kind() == 'Map')
        rebuild = [ kv.kv() for kv in person_node ]
        rebuild = { k.value(): v.value() for k,v in rebuild }
        assert rebuild == {'age': 'Int', 'name': 'Tstr', 'employer': 'Tstr'}

    def test_dicts_arrays(self):
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

        tools = [
            (cbor2.dumps, cddlcat.validate_cbor_bytes),
            (json.dumps, cddlcat.validate_json_str),
        ]

        for encode, validate in tools:
            print("test validator", validate.__name__)
            encoded_dict = encode(person2_dict)
            encoded_array = encode(person2_array)

            #print('dict after round-trip:', cbor2.loads(encoded_dict))
            #print('array after round-trip:', cbor2.loads(encoded_array))

            # CBOR dict vs CDDL dict
            self.assertEqual(None, validate('person', cddl_spec, encoded_dict))
            # CBOR array vs CDDL dict
            with self.assertRaises(Exception):
                validate('person', cddl_spec, encoded_array)

            # CBOR dict vs CDDL array
            with self.assertRaises(Exception):
                validate('person_array', cddl_spec, encoded_dict)
            # CBOR dict vs CDDL array
            assert None == validate('person_array', cddl_spec, encoded_array)

    def test_bad_arrays(self):
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
            with self.assertRaises(Exception):
                cddlcat.validate_cbor_bytes('person_array', cddl_spec, bad_cbor)


if __name__ == '__main__':
    unittest.main()
