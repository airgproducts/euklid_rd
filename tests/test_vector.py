#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors in the rust module euklid_rs"""

import math
import unittest
from euklid_rs.vector import Vector2D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''  
    
    def test_vector2d_init_values(self):
        '''test_vector2d_init_values'''
        p1 = Vector2D([0, 0])
        self.assertIs(type(p1), Vector2D)
        self.assertEqual(p1.x, 0.0)
        self.assertEqual(p1.y, 0.0)

    def test_vector2d_normalized_values(self):
        '''test_vector2d_normalized_values checks if the values'''
        p1 = Vector2D([1, 1])
        p1.normalized()
        self.assertEqual(round(p1.x, 4), 0.7071)
        self.assertEqual(round(p1.y, 4), 0.7071)

        p2 = Vector2D([0, 0])
        p2.normalized()
        self.assertTrue(math.isnan(round(p2.x, 4)))
        self.assertTrue(math.isnan(round(p2.y, 4)))

        p3 = Vector2D([2, 3])
        p3.normalized()
        self.assertEqual(round(p3.x, 4), 0.5547)
        self.assertEqual(round(p3.y, 4), 0.8321)

if __name__ == '__main__':
    unittest.main(exit=False)