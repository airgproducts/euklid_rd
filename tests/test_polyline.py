#!/usr/bin/env python3
# coding: utf-8

"""Unittest for vectors from the rust module euklid_rs"""
import random
import unittest
from euklid_rs.polyline import PolyLine2D, PolyLine3D
from euklid_rs.vector import Vector2D, Vector3D

class TestRustModules(unittest.TestCase):
    '''Test euklid_rs rust module'''
    def setUp(self) -> None:
        # Vector2D
        self.line = PolyLine2D([[0,0],[1,0],[2,1],[1,3],[0,0]])

    def test_get(self):
        self.assertEqual(self.line.get(0), Vector2D([0,0]))
        self.assertEqual(self.line.get(1), Vector2D([1,0]))
        self.assertEqual(self.line.get(1.5), Vector2D([1.5,0.5]))
        self.assertEqual(self.line.get(-1), Vector2D([-1,0]))
    
    def test_list_access(self):
        self.assertEqual(self.line[0], self.line[-len(self.line)])
        self.assertEqual(len(self.line), 5)

    def test_walk(self):
        self.assertEqual(self.line.walk(0, 1), 1)
        self.assertEqual(self.line.walk(0, self.line.get_length()), len(self.line)-1)
        self.assertEqual(self.line.walk(0, -1), -1)
        self.assertAlmostEqual(self.line.walk(0, 0.1), 0.1)
    
    def test_resample(self):
        target_length = random.randint(20, 500)
        resampled_line = self.line.resample(target_length)
        self.assertEqual(resampled_line[0], self.line[0])
        self.assertEqual(resampled_line[-1], self.line[-1])
        self.assertEqual(len(resampled_line), target_length)
        self.assertAlmostEqual(self.line.resample(1900).get_length(), self.line.get_length(),1)
    
    def test_cut(self):
        p1 = Vector2D([0.5, -1])
        p2 = Vector2D([0.5, 1])
        print(self.line.cut(p1, p2))
        self.assertAlmostEqual(self.line.cut(p1, p2)[0].ik_1, 0.5)


if __name__ == '__main__':
    unittest.main(exit=False)
