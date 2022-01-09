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
        self.assertEqual(str(p1), 'Vector2D(0.0000 0.0000)')

    def test_vector2d_normalized_values(self):
        '''test_vector2d_normalized_values checks if the values'''
        p1 = Vector2D([1, 1])

        self.assertEqual(str(p1.normalized()), 'Vector2D(0.7071 0.7071)')

        p2 = Vector2D([0, 0])
        self.assertTrue(math.isnan(p2.normalized().x))
        self.assertTrue(math.isnan(p2.normalized().y))


        p3 = Vector2D([2, 3])
        self.assertEqual(round(p3.normalized().x, 4), 0.5547)
        self.assertEqual(round(p3.normalized().y, 4), 0.8321)

if __name__ == '__main__':
    unittest.main(exit=False)