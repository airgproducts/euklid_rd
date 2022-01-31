#!/usr/bin/env python3
# coding: utf-8

"""Unittest for euklid_rs against euklid"""

import math
import unittest
import euklid
import euklid_rs


class TestVectorFunctions(unittest.TestCase):
    """Test euklid_rs.vector against euklid.vector"""

    def setUp(self) -> None:
        # Vector2D
        self.c_p2d_1 = euklid.vector.Vector2D([2, 3])
        self.c_p2d_2 = euklid.vector.Vector2D([-4, -3])
        self.r_p2d_1 = euklid_rs.vector.Vector2D([2, 3])
        self.r_p2d_2 = euklid_rs.vector.Vector2D([-4, -3])

        # Vector3D
        self.c_p3d_1 = euklid.vector.Vector3D([2, 3, 4])
        self.c_p3d_2 = euklid.vector.Vector3D([-4, -3, -2])
        self.r_p3d_1 = euklid_rs.vector.Vector3D([2, 3, 4])
        self.r_p3d_2 = euklid_rs.vector.Vector3D([-4, -3, -2])

    def test_angle(self):
        """test_angle comparision"""
        assert self.r_p2d_1.angle() == self.c_p2d_1.angle()

    def test_cross(self):
        """test_cross comparision"""
        assert self.r_p2d_1.cross(self.r_p2d_2) == self.c_p2d_1.cross(self.c_p2d_2)
        assert str(self.r_p3d_1.cross(self.r_p3d_2)) == str(
            self.c_p3d_1.cross(self.c_p3d_2)
        )

    def test_copy(self):
        """test_copy comparision"""
        assert str(self.r_p2d_1.copy()) == str(self.c_p2d_1.copy())
        assert str(self.r_p3d_1.copy()) == str(self.c_p3d_1.copy())

    def test_dot(self):
        """test_dot comparision"""
        assert self.r_p2d_1.dot(self.r_p2d_2) == self.c_p2d_1.dot(self.c_p2d_2)
        assert self.r_p3d_1.dot(self.r_p3d_2) == self.c_p3d_1.dot(self.c_p3d_2)

    def test_length(self):
        """test_length comparision"""
        assert self.r_p2d_1.length() == self.c_p2d_1.length()
        assert self.r_p3d_1.length() == self.c_p3d_1.length()

    def test_normalized(self):
        """test_normalized comparision"""
        assert str(self.r_p2d_1.normalized()) == str(self.c_p2d_1.normalized())
        assert str(self.r_p3d_1.normalized()) == str(self.c_p3d_1.normalized())

    def test__repr__(self):
        """test__repr__ comparision"""
        assert str(self.r_p2d_1) == str(self.c_p2d_1)
        assert str(self.r_p3d_1) == str(self.c_p3d_1)


class TestVectorTransformFunctions(unittest.TestCase):
    """Test euklid_rs against euklid"""

    def setUp(self) -> None:
        self.c_p3d_1 = euklid.vector.Vector3D([3, 4, 5])
        self.c_p3d_2 = euklid.vector.Vector3D([-1, -2, -3])
        self.r_p3d_1 = euklid_rs.vector.Vector3D([3, 4, 5])
        self.r_p3d_2 = euklid_rs.vector.Vector3D([-1, -2, -3])

    def test_translation(self):
        """test_translation comparision"""
        excepted = euklid.vector.Transformation.translation(self.c_p3d_1).apply(
            self.c_p3d_2
        )
        result = euklid_rs.vector.Transformation.translation(self.r_p3d_1).apply(
            self.r_p3d_2
        )
        assert str(result) == str(excepted)

    def test_rotation(self):
        """test_rotation comparision"""
        c_axis = euklid.vector.Vector3D([1, 1, 0])
        c_rotation = euklid.vector.Transformation.rotation(math.pi, c_axis).apply(
            self.c_p3d_1
        )
        r_axis = euklid_rs.vector.Vector3D([1, 1, 0])
        r_rotation = euklid_rs.vector.Transformation.rotation(math.pi, r_axis).apply(
            self.r_p3d_1
        )
        assert str(c_rotation) == str(r_rotation)

    def test_scale(self):
        """test_scale comparision"""
        excepted = euklid.vector.Transformation.scale(0.5).apply(self.c_p3d_1).length()
        result = euklid_rs.vector.Transformation.scale(0.5).apply(self.r_p3d_1).length()
        assert result == excepted


if __name__ == "__main__":
    unittest.main(exit=False)
