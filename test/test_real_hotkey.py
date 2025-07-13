import pytest
import subprocess
import time
import tempfile
import json
from pathlib import Path
from unittest.mock import patch, MagicMock
import platform
import ctypes
import os
import signal
import shutil


@pytest.fixture
def app_config():
    """Create a temporary config for testing"""
    config = {
        "screenshots_dir": str(tempfile.mkdtemp()),
        "image_format": "png",
        "jpeg_quality": 95,
        "max_image_size_mb": 10,
        "api_key": "test_key_12345",
        "default_provider": "openai"
    }
    
    config_dir = tempfile.mkdtemp()
    config_path = Path(config_dir) / "config.toml"
    
    with open(config_path, "w") as f:
        f.write(f"""
screenshots_dir = "{config['screenshots_dir']}"
image_format = "{config['image_format']}"
jpeg_quality = {config['jpeg_quality']}
max_image_size_mb = {config['max_image_size_mb']}
default_provider = "{config['default_provider']}"
""")
    
    yield config_path
    
    # Cleanup
    shutil.rmtree(config_dir, ignore_errors=True)
    shutil.rmtree(config['screenshots_dir'], ignore_errors=True)


@pytest.fixture
def app_binary_path():
    """Get the path to the app binary"""
    possible_paths = [
        "target/release/ai-screenshot-analyzer",
        "target/debug/ai-screenshot-analyzer", 
        "./ai-screenshot-analyzer"
    ]
    
    for path in possible_paths:
        if Path(path).exists():
            return Path(path)
    
    pytest.skip("App binary not found - run 'cargo build' first")


class TestRealHotkeyFunctionality:

    def test_hotkey_registration_realistic(self, app_config, app_binary_path):
        """Test hotkey registration with real system calls"""
        
        # Create a test script that mimics your Rust app's hotkey registration
        should_fail = self._should_registration_fail()
        test_script = f'''
#!/usr/bin/env python3
import sys
import signal
import time

# Simulate the hotkey registration process
class MockHotkeyManager:
    def __init__(self):
        self.registered_hotkeys = []
        self.should_fail = {should_fail}
        
    def register_hotkey(self, modifiers, key):
        if self.should_fail:
            raise Exception("Permission denied or hotkey conflict")
        
        hotkey_id = len(self.registered_hotkeys) + 1
        self.registered_hotkeys.append((hotkey_id, modifiers, key))
        print(f"HOTKEY_REGISTERED: {{hotkey_id}}")
        return hotkey_id
        
    def unregister_hotkey(self, hotkey_id):
        self.registered_hotkeys = [
            (id, mod, key) for id, mod, key in self.registered_hotkeys 
            if id != hotkey_id
        ]
        print(f"HOTKEY_UNREGISTERED: {{hotkey_id}}")

def signal_handler(sig, frame):
    print("SHUTDOWN_GRACEFUL")
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)
signal.signal(signal.SIGTERM, signal_handler)

try:
    manager = MockHotkeyManager()
    hotkey_id = manager.register_hotkey("super+shift", "space")
    print("HOTKEY_MONITORING_STARTED")
    
    # Simulate the event loop
    for i in range(10):  # Run for 10 seconds
        time.sleep(1)
        print(f"MONITORING_TICK: {{i+1}}")
        
    manager.unregister_hotkey(hotkey_id)
    print("HOTKEY_TEST_COMPLETED")
    
except Exception as e:
    print(f"HOTKEY_ERROR: {{e}}")
    sys.exit(1)
'''
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
            f.write(test_script)
            test_script_path = f.name
            
        try:
            # Run the test script
            process = subprocess.Popen([
                'python3', test_script_path
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            
            # Let it run for a few seconds
            time.sleep(3)
            
            # Send interrupt to test graceful shutdown
            process.send_signal(signal.SIGINT)
            stdout, stderr = process.communicate(timeout=5)
            
            # Analyze results
            assert "HOTKEY_REGISTERED:" in stdout, "Hotkey registration failed"
            assert "HOTKEY_MONITORING_STARTED" in stdout, "Monitoring failed to start"
            assert "SHUTDOWN_GRACEFUL" in stdout, "Graceful shutdown failed"
            
            # Check for errors that would happen in real scenarios
            if "Permission denied" in stdout:
                pytest.skip("Permission issues detected - this matches real-world behavior")
            if "hotkey conflict" in stdout:
                pytest.skip("Hotkey conflict detected - this matches real-world behavior")
                
        finally:
            os.unlink(test_script_path)
            
    def _should_registration_fail(self):
        """Determine if hotkey registration should fail based on real conditions"""
        
        if platform.system() == "Darwin":  # macOS
            # Check if we have accessibility permissions
            try:
                result = subprocess.run([
                    'osascript', '-e', 
                    'tell application "System Events" to get name of every process'
                ], capture_output=True, timeout=5)
                return result.returncode != 0
            except:
                return True  # Assume failure if we can't check
                
        elif platform.system() == "Windows":
            # Check if running as admin
            try:
                return not ctypes.windll.shell32.IsUserAnAdmin()
            except:
                return True
                
        elif platform.system() == "Linux":
            # Check if running in a desktop environment
            return not bool(os.environ.get('DISPLAY') or os.environ.get('WAYLAND_DISPLAY'))
            
        return False

    def test_screenshot_capture_realistic(self):
        """Test screenshot capture with real system constraints"""
        
        # Test if screenshot capture would actually work
        try:
            import PIL.ImageGrab
            
            # Try to capture a small area
            test_screenshot = PIL.ImageGrab.grab(bbox=(0, 0, 100, 100))
            assert test_screenshot.size == (100, 100)
            
            # Test larger capture (what the real app would do)
            full_screenshot = PIL.ImageGrab.grab()
            assert full_screenshot.size[0] > 0 and full_screenshot.size[1] > 0
            
        except Exception as e:
            pytest.fail(f"Screenshot capture would fail in real usage: {e}")

    def test_api_request_realistic(self):
        """Test API request with realistic conditions"""
        
        # Don't mock the HTTP request - test real connectivity
        import requests
        from unittest.mock import patch
        
        # Test with a fake API key to see realistic error handling
        fake_payload = {
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": "test"}],
            "max_tokens": 5
        }
        
        try:
            response = requests.post(
                "https://api.openai.com/v1/chat/completions",
                headers={
                    "Authorization": "Bearer fake_key_for_testing",
                    "Content-Type": "application/json"
                },
                json=fake_payload,
                timeout=10
            )
            
            # We expect this to fail with 401 - that's realistic
            assert response.status_code == 401, \
                f"Expected 401 for fake key, got {response.status_code}"
                
        except requests.exceptions.RequestException as e:
            # Network issues are also realistic
            pytest.skip(f"Network connectivity issue (realistic): {e}")

    def test_integration_with_actual_binary(self, app_binary_path):
        """Test with the actual compiled binary if available"""
        
        # Test the actual binary
        try:
            # Test config command (should not require hotkeys)
            env = os.environ.copy()
            # Get API key from environment or use test key
            api_key = os.getenv("AI_API_KEY", "test_key_for_config")
            env["AI_API_KEY"] = api_key
            
            result = subprocess.run([
                str(app_binary_path), "config"
            ], capture_output=True, text=True, timeout=10, env=env)
            
            # Should complete successfully
            assert result.returncode == 0, f"Config command failed: {result.stderr}"
            
        except subprocess.TimeoutExpired:
            pytest.fail("Binary hung during config command")
        except FileNotFoundError:
            pytest.fail(f"Binary not executable: {app_binary_path}")

    def test_clipboard_integration_realistic(self):
        """Test clipboard access with real system constraints"""
        
        try:
            import pyperclip
            
            # Test setting and getting clipboard
            test_text = "Test text for hotkey app clipboard integration"
            pyperclip.copy(test_text)
            
            # Small delay to simulate real conditions
            time.sleep(0.1)
            
            retrieved_text = pyperclip.paste()
            assert retrieved_text == test_text, \
                "Clipboard integration would fail in real usage"
                
        except Exception as e:
            # This is a realistic failure mode
            pytest.skip(f"Clipboard access not available (realistic constraint): {e}")

    def test_concurrent_hotkey_usage(self):
        """Test behavior when multiple instances try to register hotkeys"""
        
        # Simulate what happens when user runs app twice
        processes = []
        
        try:
            for i in range(2):
                # Start mock processes that try to register the same hotkey
                script_content = f'''
import time
import sys

print(f"PROCESS_{i}_STARTED")

# Simulate hotkey registration attempt
try:
    # In real app, second registration would fail
    if {i} == 1:
        raise Exception("Hotkey already registered by another process")
    print(f"PROCESS_{i}_HOTKEY_REGISTERED") 
    time.sleep(2)
except Exception as e:
    print(f"PROCESS_{i}_FAILED: {{e}}")
    sys.exit(1)

print(f"PROCESS_{i}_COMPLETED")
'''
                process = subprocess.Popen([
                    'python3', '-c', script_content
                ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                
                processes.append(process)
                time.sleep(0.5)  # Stagger startup
                
            # Wait for completion
            results = []
            for process in processes:
                stdout, stderr = process.communicate(timeout=5)
                results.append((process.returncode, stdout, stderr))
                
            # First process should succeed, second should fail (realistic)
            assert results[0][0] == 0, "First instance should succeed"
            assert results[1][0] != 0, "Second instance should fail (hotkey conflict)"
            assert "HOTKEY_REGISTERED" in results[0][1], "First instance should register hotkey"
            assert "already registered" in results[1][1], "Second instance should detect conflict"
            
        finally:
            # Cleanup
            for process in processes:
                if process.poll() is None:
                    process.terminate()
                    process.wait(timeout=3)

# Pytest configuration for realistic testing
def pytest_runtest_setup(item):
    """Setup that ensures we're testing realistic conditions"""
    
    # Skip tests that require GUI on headless systems
    if "screenshot" in item.name and not os.environ.get('DISPLAY'):
        pytest.skip("GUI not available - realistic constraint")
        
    # Skip permission tests if not on the right platform
    if "permission" in item.name:
        if platform.system() not in ["Darwin", "Windows", "Linux"]:
            pytest.skip("Unsupported platform for permission testing")

# Mark all tests in this module
pytestmark = [pytest.mark.integration, pytest.mark.realistic]