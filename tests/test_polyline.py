#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""
import random
import unittest
from euklid_rs.polyline import PolyLine2D
from euklid_rs.vector import Vector2D


class TestRustModules(unittest.TestCase):
    """Test euklid_rs rust module"""

    def setUp(self) -> None:
        # Vector2D
        self.line = PolyLine2D([[0, 0], [1, 0], [2, 1], [3, 3], [4, 5]])

    def test_get(self):
        """Test getting a point"""
        self.assertEqual(self.line.get(0), Vector2D([0, 0]))
        self.assertEqual(self.line.get(1), Vector2D([1, 0]))
        self.assertEqual(self.line.get(1.5), Vector2D([1.5, 0.5]))
        self.assertEqual(self.line.get(-1), Vector2D([-1, 0]))

    def test_list_access(self):
        """test polyline getitem"""
        self.assertEqual(self.line[0], self.line[-len(self.line)])
        self.assertEqual(len(self.line), 5)

    def test_walk(self):
        """test polyline walking"""
        self.assertEqual(self.line.walk(0, 1), 1)
        self.assertEqual(self.line.walk(0, self.line.get_length()), len(self.line) - 1)
        self.assertEqual(self.line.walk(0, -1), -1)
        self.assertAlmostEqual(self.line.walk(0, 0.1), 0.1)

    def test_resample(self):
        """test resampling"""
        target_length = random.randint(20, 500)
        resampled_line = self.line.resample(target_length)
        self.assertEqual(resampled_line[0], self.line[0])
        self.assertEqual(resampled_line[-1], self.line[-1])
        self.assertEqual(len(resampled_line), target_length)
        self.assertAlmostEqual(resampled_line.get_length(), self.line.get_length(), 2)

    # pylint: disable=invalid-name
    def assertSingleCut(self, cuts, ik_1: float):
        """Check if there is only one cut and if it is at a certain position"""
        self.assertEqual(len(cuts), 1)
        self.assertAlmostEqual(cuts[0].ik_1, ik_1)

    def test_cut(self):
        """Test cutting the line with two nodes"""
        p1 = Vector2D([0.5, -1])
        p2 = Vector2D([0.5, 1])
        self.assertSingleCut(self.line.cut(p1, p2), 0.5)

    def test_cut_on_node(self):
        """Test cutting directly at a node"""
        p2_1 = Vector2D([1, -1])
        p2_2 = Vector2D([1, 1])
        self.assertSingleCut(self.line.cut(p2_1, p2_2), 1.0)

    def test_complicated_cut(self):
        """test edge case (tolerance)"""
        curve = PolyLine2D(
            [
                [-0.10104283355824818, -0.1923445451803646],
                [2.7646271914588056, -0.2998482674465057],
                [2.9331737845193144, -0.3375670982030803],
                [4.176588348946094, -0.693168305324235],
                [4.36715677382829, -0.7732455672374599],
            ]
        )

        p1 = Vector2D([4.176588348946094, -0.6066228276490806])
        p2 = Vector2D([4.176588348946094, -1.3933998974232116])

        cuts = curve.cut(p1, p2)
        self.assertSingleCut(cuts, 1.)


if __name__ == "__main__":
    unittest.main(exit=False)
