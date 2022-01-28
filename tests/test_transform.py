import imp
import unittest
import math
from euklid_rs.vector import *


class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        # Vector3D
        self.p3d_1 = Vector3D([0,0,0])
        self.p3d_2 = Vector3D([1,1,1])
        self.p3d_3 = Vector3D([2,3,4])
    
    def asertAlmostEqualVec(self, v1, v2):
        try:
            for i in range(3):
                self.assertAlmostEqual(v1[i], v2[i])
        except AssertionError:
            raise AssertionError(f"{v1} != {v2}")

    def test_translation(self):
        translation = Transformation.translation(self.p3d_2)
        self.assertNotEqual(self.p3d_2, translation.apply(self.p3d_2))

        self.assertEqual(self.p3d_2, translation.apply(self.p3d_1))

    def test_rotation(self):
        axis = Vector3D([1,1,0])
        rotation = Transformation.rotation(math.pi, axis)

        self.assertEqual(self.p3d_1, rotation.apply(self.p3d_1))

        vec1 = Vector3D([0, 1., 1.])
        vec2 = Vector3D([1, 0., -1.])
        self.asertAlmostEqualVec(rotation.apply(vec1), vec2)

    def test_scale(self):
        scale = 0.5
        transform = Transformation.scale(0.5)

        self.assertEqual(self.p3d_2.length()*scale, transform.apply(self.p3d_2).length())