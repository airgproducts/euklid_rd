#!/usr/bin/env python3
# coding: utf-8

"""Unittest for the rust module spatial_rust_util"""

import unittest
from euklid_rs import hello_world

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    
    def test_hello_world(self):
        '''test_hello_world should return Hello, world!'''
        result = hello_world()
        self.assertEqual(result, 'Hello, world!')

if __name__ == '__main__':
    unittest.main(exit=False)