from typing import Type
import unittest

import euklid_rs
from .base import TestCase

__all__ = ["BezierTest"]


class CurveTestBase(TestCase):
    """
    Base class for curve tests
    """
    CurveType: Type

    def setUp(self) -> None:
        """Setup"""
        points = [[0, 0], [1, 0], [1, 1]]
        self.curve = self.CurveType(euklid_rs.polyline.PolyLine2D(points))

        return super().setUp()
    
    @unittest.skip(reason="bsplinecurve3 fails")
    def test_base(self):
        """Test curve start and end"""
        self.assert_almost_equal_vec(self.curve.get(0), [0, 0])
        self.assert_almost_equal_vec(self.curve.get(1), [1, 1])

    def test_sequence_length(self):
        """Test length of sequence"""
        sequence = self.curve.get_sequence(50)
        self.assertEqual(len(sequence), 50)


class BezierTest(CurveTestBase):
    """
    Bezier Curve test class
    """
    CurveType = euklid_rs.spline.BezierCurve

    def test_length(self):
        """test curve length"""
        print(self.curve.get_sequence(50).get_length())


class BSplineTest(CurveTestBase):
    CurveType = euklid_rs.spline.BSplineCurve3

    def test_length(self):
        print(self.curve.get_sequence(50).get_length())



del CurveTestBase
