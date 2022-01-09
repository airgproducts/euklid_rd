#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors in the rust module euklid_rs"""

import math
import unittest
from euklid_rs.vector import Vector2D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        self.p1 = Vector2D([0,0])
        self.p2 = Vector2D([1,1])
        self.p3 = Vector2D([2,3])
    
    def test_vector2d_init_values(self):
        '''test_vector2d_init_values'''
        self.assertIs(type(self.p1), Vector2D)
        self.assertEqual(self.p1[0], 0.0)
        self.assertEqual(self.p1[1], 0.0)
        self.assertEqual(str(self.p1), 'Vector2D(0.0000 0.0000)')
    
    def test_norm(self):
        self.assertAlmostEqual(self.p1.length(), 0)
        self.assertAlmostEqual(self.p2.length(), math.sqrt(2))
        self.assertAlmostEqual(self.p3.length(), math.sqrt(13))

    def test_vector2d_normalized_values(self):
        '''test_vector2d_normalized_values checks if the values'''
        self.assertAlmostEqual(self.p2.normalized().length(), 1)
        self.assertAlmostEqual(self.p3.normalized().length(), 1)

        self.assertTrue(math.isnan(self.p1.normalized()[0]))
        self.assertTrue(math.isnan(self.p1.normalized()[1]))

        self.assertEqual(round(self.p3.normalized()[0], 4), 0.5547)
        self.assertEqual(round(self.p3.normalized()[1], 4), 0.8321)
    
    def test_access_invalid(self):
        self.assertRaises(IndexError, lambda: self.p1[2])
        self.assertRaises(IndexError, lambda: self.p1[-1])

if __name__ == '__main__':
    unittest.main(exit=False)