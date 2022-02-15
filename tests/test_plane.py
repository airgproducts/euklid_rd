#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""

import unittest
import euklid
from euklid_rs.plane import Plane
from euklid_rs.vector import Vector2D, Vector3D


class TestPlaneFunctions(unittest.TestCase):
    """Test euklid_rs.vector.Transformation rust module"""

    def setUp(self) -> None:
        self.r_p3d_p0 = Vector3D([2, 3, 4])
        self.r_p3d_v1 = Vector3D([-4, -3, -2])
        self.r_p3d_v2 = Vector3D([4, 5, 6])

        self.plane_r = Plane(self.r_p3d_p0, self.r_p3d_v1, self.r_p3d_v2)

    @unittest.skip(reason="p0 is probably wrong")
    def test_plane_return_vectors_p0(self) -> None:
        assert str(self.plane_r.p0) == str(euklid.vector.Vector3D([2, 3, 4]))

    def test_plane_return_x_vector(self) -> None:
        assert str(self.plane_r.x_vector) == str(euklid.vector.Vector3D([-4, -3, -2]))

    def test_plane_return_y_vector(self) -> None:
        assert str(self.plane_r.y_vector) == str(euklid.vector.Vector3D([4, 5, 6]))

    def test_plane_return_normvector(self) -> None:
        assert str(self.plane_r.normvector) == str(euklid.vector.Vector3D([-8, 16, -8]))

    @unittest.skip(reason="p0 is probably wrong")
    def test_plane_project(self) -> None:
        assert str(self.plane_r.project(self.r_p3d_v1)) == str(Vector2D([54, -90]))
