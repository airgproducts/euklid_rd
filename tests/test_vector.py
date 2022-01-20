#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors in the rust module euklid_rs"""

import math
import unittest
from euklid_rs.vector import Vector2D
from euklid_rs.vector import Vector3D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        self.p1 = Vector2D([0,0])
        self.p2 = Vector2D([1,1])
        self.p3 = Vector2D([2,3])

    def test_angle(self):
        '''test_angle of a vector'''
        self.assertEqual(round(self.p2.angle(), 4), 0.7854)
        self.assertEqual(round(self.p3.angle(), 4), 0.9828)

    def test_copy(self):
        '''test_copy a vector'''
        self.assertEqual(str(self.p1), 'Vector2D(0.0000 0.0000)')
        result = self.p1.copy()
        self.assertEqual(str(result), 'Vector2D(0.0000 0.0000)')

    def test_cross(self):
        '''test_cross product of a vector'''
        self.assertEqual(round(self.p2.cross(self.p3), 4), 1.)
        self.assertEqual(round(self.p3.cross(self.p2), 4), -1)

    def test_dot(self):
        '''test_dot of two vectors'''
        self.assertEqual(round(self.p2.dot(self.p3), 4), 5.)
        self.assertEqual(round(self.p3.dot(self.p2), 4), 5)

    def test_vector2d_init_values(self):
        '''test_vector2d_init_values'''
        self.assertIs(type(self.p1), Vector2D)
        self.assertEqual(self.p1[0], 0.0)
        self.assertEqual(self.p1[1], 0.0)
        self.assertEqual(str(self.p1), 'Vector2D(0.0000 0.0000)')

    def test_length(self):
        '''test_length get the length of a vector'''
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
        '''test_access_invalid raises a error'''
        self.assertRaises(IndexError, lambda: self.p1[2])
        self.assertRaises(IndexError, lambda: self.p1[-1])

    def test__setitem__(self):
        '''test__setitem__ sets a value at a position'''
        self.p1[0] = 2
        self.assertEqual(self.p1[0], 2.)

    def test__add__(self):
        '''test__add__ tries to add two vectors'''
        self.assertEqual(self.p2 + self.p3, Vector2D([3, 4]))

    def test__sub__(self):
        '''test__add__ tries to subtract two vectors'''
        self.assertEqual(self.p2 - self.p3, Vector2D([-1, -2]))

    def test__mul__(self):
        '''test__mul__ multiplies a vector'''
        self.assertEqual(self.p3 * 3, Vector2D([6, 9]))

    def test__truediv__(self):
        '''test__truediv__ devides a vector'''
        self.assertEqual(self.p3 / 2, Vector2D([1, 1.5]))

    def test_compare(self):
        '''test_compare tests the comparison of two vectors'''
        short = self.p2 * 0.8
        long = self.p2 / 0.8

        self.assertTrue(self.p2 == self.p2)

        self.assertTrue(short < self.p2)
        self.assertTrue(short <= self.p2)

        self.assertTrue(long > self.p2)
        self.assertTrue(long >= self.p2)

        self.assertFalse(short > self.p2)
        self.assertFalse(short >= self.p2)

        self.assertFalse(long < self.p2)
        self.assertFalse(long <= self.p2)

    def test_docstring(self):
        '''test_docstring returns the docstring of a function'''
        self.assertEqual(self.p2.copy.__doc__, 'This function copies a Vector2D object.')


class TestRustModulesVector3D(unittest.TestCase):
    '''Test euklid_rs Vector3D rust module'''
    def setUp(self) -> None:
        self.p1 = Vector3D([0,0,0])
        self.p2 = Vector3D([1,1,1])
        self.p3 = Vector3D([2,3,4])

    def test_copy(self):
        '''test_copy a vector'''
        self.assertEqual(str(self.p1), 'Vector3D(0.0000 0.0000 0.0000)')
        result = self.p1.copy()
        self.assertEqual(str(result), 'Vector3D(0.0000 0.0000 0.0000)')

if __name__ == '__main__':
    unittest.main(exit=False)
