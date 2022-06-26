#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""

from itertools import chain
import math
from euklid_rs.vector import Vector3D, Transformation

from .base import TestCase


class TestVectorTransFormFunctions(TestCase):
    """Test euklid_rs.vector.Transformation rust module"""

    def setUp(self) -> None:
        self.vectors = [
            Vector3D([0, 0, 0]),
            Vector3D([1, 1, 1]),
            Vector3D([2, 3, 4])
        ]
    
    def get_transformations(self):
        transformations = [
            Transformation.translation(self.vectors[1]),
            Transformation.rotation(math.pi/4, self.vectors[1]),
            Transformation.scale(0.3)
        ]
        transformations.append(transformations[0].chain(transformations[1]).chain(transformations[2]))

        return transformations

    def test_translation(self):
        """Check if translation works"""
        translation = Transformation.translation(self.vectors[1])
        self.assertNotEqual(self.vectors[1], translation.apply(self.vectors[1]))

        self.assertEqual(self.vectors[1], translation.apply(self.vectors[0]))
        self.assertEqual(translation.apply(self.vectors[1]), self.vectors[1]*2)

    def test_rotation(self):
        """Check if rotation works"""
        axis = Vector3D([1, 1, 0])
        rotation = Transformation.rotation(math.pi, axis)
        self.assertEqual(self.vectors[0], rotation.apply(self.vectors[0]))
        self.assert_almost_equal_vec(rotation.apply(Vector3D([1, 0.0, -1.0])), Vector3D([0, 1.0, 1.0]))

    def test_scale(self):
        for scale_amount in (0.1, 1., 10, 100):
            scale = Transformation.scale(scale_amount)
            for p in (self.vectors[0], self.vectors[1], self.vectors[2]):
                self.assert_almost_equal_vec(scale.apply(p), p*scale_amount)

    def test_chain(self):
        translation_vec = Vector3D([1,0,0])
        translation = Transformation.translation(translation_vec)

        rotation_axis = Vector3D([0,0,1])
        rotation = Transformation.rotation(math.pi/2, rotation_axis)

        chain1 = rotation.chain(translation) # translate, then rotate
        chain2 = translation.chain(rotation) # rotate, then translate

        # [0,0,0]
        self.assert_almost_equal_vec(chain1.apply(self.vectors[0]), [0,1,0])
        self.assert_almost_equal_vec(chain2.apply(self.vectors[0]), [1,0,0])

        # [1,1,1]
        self.assert_almost_equal_vec(chain1.apply(self.vectors[1]), [-1, 2, 1])
        self.assert_almost_equal_vec(chain2.apply(self.vectors[1]), [0, 1, 1])

        # [2,3,4]
        self.assert_almost_equal_vec(chain1.apply(self.vectors[2]), [-3, 3, 4])
        self.assert_almost_equal_vec(chain2.apply(self.vectors[2]), [-2, 2, 4])


    def test_inverse(self):
        for transformation in self.get_transformations():
            for v in self.vectors:
                self.assert_almost_equal_vec(transformation.apply_inverse(transformation.apply(v)), v)
