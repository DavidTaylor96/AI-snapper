#!/usr/bin/env python3
"""
Python test runner for AI Screenshot Analyzer
Runs the hotkey test suites with proper configuration
"""

import sys
import subprocess
import os
from pathlib import Path


def install_dependencies():
    """Install required Python packages for testing"""
    required_packages = [
        "pytest",
        "psutil", 
        "pyperclip",
        "Pillow",
        "requests"
    ]
    
    print("Installing required dependencies...")
    
    # Check if we're in a virtual environment
    venv_active = hasattr(sys, 'real_prefix') or (hasattr(sys, 'base_prefix') and sys.base_prefix != sys.prefix)
    
    if not venv_active:
        print("No virtual environment detected. Please activate test_venv:")
        print("  source test_venv/bin/activate")
        return False
    
    for package in required_packages:
        try:
            subprocess.run([
                sys.executable, "-m", "pip", "install", package
            ], check=True, capture_output=True)
            print(f"✓ {package}")
        except subprocess.CalledProcessError:
            print(f"✗ Failed to install {package}")
            return False
    
    return True


def build_rust_binary():
    """Build the Rust binary for integration tests"""
    print("Building Rust binary...")
    try:
        result = subprocess.run([
            "cargo", "build", "--release"
        ], check=True, capture_output=True, text=True)
        print("✓ Rust binary built successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to build Rust binary: {e.stderr}")
        return False


def run_tests():
    """Run the Python test suites"""
    test_dir = Path("test")
    if not test_dir.exists():
        print("✗ Test directory not found")
        return False
    
    # Set up environment
    env = os.environ.copy()
    env["PYTHONPATH"] = str(Path.cwd())
    
    # Run test_hotkey_suite
    print("\nRunning test_hotkey_suite...")
    try:
        result = subprocess.run([
            sys.executable, "-m", "pytest", 
            "test/test_hotkey_suite.py",
            "-v",
            "--tb=short"
        ], env=env, check=True)
        print("✓ test_hotkey_suite passed")
    except subprocess.CalledProcessError:
        print("✗ test_hotkey_suite failed")
        return False
    
    # Run test_real_hotkey  
    print("\nRunning test_real_hotkey...")
    try:
        result = subprocess.run([
            sys.executable, "-m", "pytest",
            "test/test_real_hotkey.py", 
            "-v",
            "--tb=short"
        ], env=env, check=True)
        print("✓ test_real_hotkey passed")
    except subprocess.CalledProcessError:
        print("✗ test_real_hotkey failed")
        return False
    
    return True


def main():
    """Main test runner"""
    print("AI Screenshot Analyzer - Python Test Runner")
    print("=" * 50)
    
    # Install dependencies
    if not install_dependencies():
        print("Failed to install dependencies")
        sys.exit(1)
    
    # Build binary for integration tests
    if not build_rust_binary():
        print("Failed to build Rust binary")
        sys.exit(1)
    
    # Run tests
    if not run_tests():
        print("\n✗ Some tests failed")
        sys.exit(1)
    
    print("\n✓ All tests passed!")


if __name__ == "__main__":
    main()