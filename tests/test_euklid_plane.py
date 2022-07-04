#!/usr/bin/env python3
# coding: utf-8

"""Unittest for euklid_rs.plane against euklid.plane"""

import unittest
import euklid
import euklid_rs


class TestPlaneFunctions(unittest.TestCase):
    """Test euklid_rs.plane against euklid.plane"""

    def setUp(self) -> None:
        self.c_p3d_p0 = euklid.vector.Vector3D([2, 3, 4])
        self.c_p3d_v1 = euklid.vector.Vector3D([-4, -3, -2])
        self.c_p3d_v2 = euklid.vector.Vector3D([4, 5, 6])
        self.r_p3d_p0 = euklid_rs.vector.Vector3D([2, 3, 4])
        self.r_p3d_v1 = euklid_rs.vector.Vector3D([-4, -3, -2])
        self.r_p3d_v2 = euklid_rs.vector.Vector3D([4, 5, 6])

    def test_plane_args_vectors(self) -> None:
        """Test plane init args"""
        plane_c = euklid.plane.Plane(self.c_p3d_p0, self.c_p3d_v1, self.c_p3d_v2)
        plane_r = euklid_rs.plane.Plane(self.r_p3d_p0, self.r_p3d_v1, self.r_p3d_v2)
        assert str(plane_c.p0) == str(euklid.vector.Vector3D([2, 3, 4]))
        assert str(plane_c.x_vector) == str(euklid.vector.Vector3D([-4, -3, -2]))
        assert str(plane_c.y_vector) == str(euklid.vector.Vector3D([4, 5, 6]))
        assert str(plane_c.normvector) == str(euklid.vector.Vector3D([-8, 16, -8]))

        # assert str(plane_r.p0) == str(euklid.vector.Vector3D([2, 3, 4]))
        assert str(plane_r.x_vector) == str(euklid.vector.Vector3D([-4, -3, -2]))
        assert str(plane_r.y_vector) == str(euklid.vector.Vector3D([4, 5, 6]))
        assert str(plane_r.normvector) == str(euklid.vector.Vector3D([-8, 16, -8]))

    def test_plane_project(self) -> None:
        """Test plane Projection"""
        plane_c = euklid.plane.Plane(self.c_p3d_p0, self.c_p3d_v1, self.c_p3d_v2)
        assert str(plane_c.project(self.c_p3d_v1)) == str(
            euklid.vector.Vector2D([54, -90])
        )

        plane_r = euklid_rs.plane.Plane(self.r_p3d_p0, self.r_p3d_v1, self.r_p3d_v2)
        assert str(plane_r.project(self.r_p3d_v1)) == str(
            euklid_rs.vector.Vector2D([54, -90])
        )


if __name__ == "__main__":
    unittest.main(exit=False)
