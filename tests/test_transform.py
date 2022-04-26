#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""

import unittest
import math
from euklid_rs.vector import Vector3D, Transformation

from .base import TestCase


class TestVectorTransFormFunctions(TestCase):
    """Test euklid_rs.vector.Transformation rust module"""
    def setUp(self) -> None:
        # Vector3D
        self.p3d_1 = Vector3D([0,0,0])
        self.p3d_2 = Vector3D([1,1,1])
        self.p3d_3 = Vector3D([2,3,4])

    def test_translation(self):
        """Check if translation works"""
        translation = Transformation.translation(self.p3d_2)
        self.assertNotEqual(self.p3d_2, translation.apply(self.p3d_2))

        self.assertEqual(self.p3d_2, translation.apply(self.p3d_1))

    def test_rotation(self):
        """Check if rotation works"""
        axis = Vector3D([1,1,0])
        rotation = Transformation.rotation(math.pi, axis)
        self.assertEqual(self.p3d_1, rotation.apply(self.p3d_1))
        assert str(rotation.apply(Vector3D([1, 0.0, -1.0]))) == str(Vector3D([0, 1.0, 1.0]))

    def test_scale(self):
        """Check if scaling works"""
        transform = Transformation.scale(0.5)
        assert transform.apply(self.p3d_2).length() == self.p3d_2.length() * 0.5
