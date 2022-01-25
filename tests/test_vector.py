#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""

import math
import unittest
from euklid_rs.vector import Vector2D
from euklid_rs.vector import Vector3D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        # Vector2D
        self.p2d_1 = Vector2D([0,0])
        self.p2d_2 = Vector2D([1,1])
        self.p2d_3 = Vector2D([2,3])

        # Vector3D
        self.p3d_1 = Vector3D([0,0,0])
        self.p3d_2 = Vector3D([1,1,1])
        self.p3d_3 = Vector3D([2,3,4])

    def test_angle(self):
        '''test_angle of a vector'''
        self.assertEqual(round(self.p2d_2.angle(), 4), 0.7854)
        self.assertEqual(round(self.p2d_3.angle(), 4), 0.9828)

        # TODO Vector3D is needed?

    def test_copy(self):
        '''test_copy a vector'''
        # Vector2D
        self.assertEqual(str(self.p2d_1), 'Vector2D(0.0000 0.0000)')
        result = self.p2d_1.copy()
        self.assertEqual(str(result), 'Vector2D(0.0000 0.0000)')

        # Vector3D
        self.assertEqual(str(self.p3d_1), 'Vector3D(0.0000 0.0000 0.0000)')
        result = self.p3d_1.copy()
        self.assertEqual(str(result), 'Vector3D(0.0000 0.0000 0.0000)')

    def test_cross(self):
        '''test_cross product of a vector'''
        # # TODO https://github.com/airgproducts/euklid/blob/master/src/vector/vector.cpp correct implementation? https://www.mathsisfun.com/algebra/vectors-cross-product.html?
        # Vector2D
        self.assertEqual(round(self.p2d_2.cross(self.p2d_3), 4), 1.)
        self.assertEqual(round(self.p2d_3.cross(self.p2d_2), 4), -1)

        # Vector3D
        self.assertEqual(self.p3d_2.cross(self.p3d_3), Vector3D([1, -2, 1]))
        self.assertEqual(self.p3d_3.cross(self.p3d_2), Vector3D([-1, 2, -1]))

    def test_dot(self):
        '''test_dot of two vectors'''
        # Vector2D
        self.assertEqual(round(self.p2d_2.dot(self.p2d_3), 4), 5.)
        self.assertEqual(round(self.p2d_3.dot(self.p2d_2), 4), 5)

        # Vector3D
        self.assertEqual(round(self.p3d_2.dot(self.p3d_3), 4), 9.)
        self.assertEqual(round(self.p3d_3.dot(self.p3d_2), 4), 9)

    def test_init_values(self):
        '''test_init_values'''
        # Vector2D
        self.assertIs(type(self.p2d_1), Vector2D)
        self.assertEqual(self.p2d_1[0], 0.0)
        self.assertEqual(self.p2d_1[1], 0.0)
        self.assertEqual(str(self.p2d_1), 'Vector2D(0.0000 0.0000)')

        # Vector3D
        self.assertIs(type(self.p3d_1), Vector3D)
        self.assertEqual(self.p3d_1[0], 0.0)
        self.assertEqual(self.p3d_1[1], 0.0)
        self.assertEqual(str(self.p3d_1), 'Vector3D(0.0000 0.0000 0.0000)')

    def test_length(self):
        '''test_length get the length of a vector'''
        # Vector2D
        self.assertAlmostEqual(self.p2d_1.length(), 0)
        self.assertAlmostEqual(self.p2d_2.length(), math.sqrt(2))
        self.assertAlmostEqual(self.p2d_3.length(), math.sqrt(13))

        # Vector3D
        self.assertAlmostEqual(self.p3d_1.length(), 0)
        self.assertAlmostEqual(self.p3d_2.length(), math.sqrt(3))
        self.assertAlmostEqual(self.p3d_3.length(), math.sqrt(29))

    def test_normalized_values(self):
        '''test_normalized_values checks if the values'''
        # Vector2D
        self.assertAlmostEqual(self.p2d_2.normalized().length(), 1)
        self.assertAlmostEqual(self.p2d_3.normalized().length(), 1)
        # Vector3D
        self.assertAlmostEqual(self.p3d_2.normalized().length(), 1)
        self.assertAlmostEqual(self.p3d_3.normalized().length(), 1)

        # Vector2D
        self.assertTrue(math.isnan(self.p2d_1.normalized()[0]))
        self.assertTrue(math.isnan(self.p2d_1.normalized()[1]))
        # Vector3D
        self.assertTrue(math.isnan(self.p3d_1.normalized()[0]))
        self.assertTrue(math.isnan(self.p3d_1.normalized()[1]))

        # Vector2D
        self.assertEqual(round(self.p2d_3.normalized()[0], 4), 0.5547)
        self.assertEqual(round(self.p2d_3.normalized()[1], 4), 0.8321)
        # Vector3D
        self.assertEqual(round(self.p3d_3.normalized()[0], 4), 0.3714)
        self.assertEqual(round(self.p3d_3.normalized()[1], 4), 0.5571)

    def test_access_invalid(self):
        '''test_access_invalid raises a error'''
        # Vector2D
        self.assertRaises(IndexError, lambda: self.p2d_1[2])
        self.assertRaises(IndexError, lambda: self.p2d_1[-1])

        # Vector3D
        self.assertRaises(IndexError, lambda: self.p3d_1[3])
        self.assertRaises(IndexError, lambda: self.p3d_1[-1])

    @unittest.skip("invalid test")
    def test__setitem__(self):
        '''test__setitem__ sets a value at a position'''
        # Vector2D
        self.p2d_1[0] = 2
        self.assertEqual(self.p2d_1[0], 2.)

        # Vector3D
        # TODO Idx > 1 leads to a 'IndexError: index out of range' error
        self.p3d_1[1] = 3
        self.assertEqual(self.p3d_1[1], 3.)

    def test__add__(self):
        '''test__add__ tries to add two vectors'''
        self.assertEqual(self.p2d_2 + self.p2d_3, Vector2D([3, 4]))
        self.assertEqual(self.p3d_2 + self.p3d_3, Vector3D([3, 4, 5]))

    def test__sub__(self):
        '''test__add__ tries to subtract two vectors'''
        self.assertEqual(self.p2d_2 - self.p2d_3, Vector2D([-1, -2]))
        self.assertEqual(self.p3d_2 - self.p3d_3, Vector3D([-1, -2, -3]))

    def test__mul__(self):
        '''test__mul__ multiplies a vector'''
        self.assertEqual(self.p2d_3 * 3, Vector2D([6, 9]))
        self.assertEqual(self.p3d_3 * 3, Vector3D([6, 9, 12]))

    def test__truediv__(self):
        '''test__truediv__ devides a vector'''
        self.assertEqual(self.p2d_3 / 2, Vector2D([1, 1.5]))
        self.assertEqual(self.p3d_3 / 2, Vector3D([1, 1.5, 2]))

    def test_compare(self):
        '''test_compare tests the comparison of two vectors'''
        # Vector2D
        short = self.p2d_2 * 0.8
        long = self.p2d_2 / 0.8

        self.assertTrue(self.p2d_2 == self.p2d_2)

        self.assertTrue(short < self.p2d_2)
        self.assertTrue(short <= self.p2d_2)

        self.assertTrue(long > self.p2d_2)
        self.assertTrue(long >= self.p2d_2)

        self.assertFalse(short > self.p2d_2)
        self.assertFalse(short >= self.p2d_2)

        self.assertFalse(long < self.p2d_2)
        self.assertFalse(long <= self.p2d_2)

        # Vector3D
        short = self.p3d_2 * 0.8
        long = self.p3d_2 / 0.8

        self.assertTrue(self.p3d_2 == self.p3d_2)

        self.assertTrue(short < self.p3d_2)
        self.assertTrue(short <= self.p3d_2)

        self.assertTrue(long > self.p3d_2)
        self.assertTrue(long >= self.p3d_2)

        self.assertFalse(short > self.p3d_2)
        self.assertFalse(short >= self.p3d_2)

        self.assertFalse(long < self.p3d_2)
        self.assertFalse(long <= self.p3d_2)

    def test_docstring(self):
        '''test_docstring returns the docstring of a function'''
        self.assertEqual(self.p2d_2.copy.__doc__, 'This function copies a Vector object.')


if __name__ == '__main__':
    unittest.main(exit=False)
