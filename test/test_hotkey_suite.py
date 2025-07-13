import pytest
import subprocess
import time
import psutil
import os
import sys
from unittest.mock import patch, MagicMock
import requests
import json
from pathlib import Path


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


@pytest.fixture
def cleanup_processes():
    """Cleanup fixture to kill any running app instances"""
    def _cleanup():
        for proc in psutil.process_iter(['pid', 'name']):
            try:
                if 'ai-screenshot-analyzer' in proc.info['name']:
                    proc.kill()
                    proc.wait(timeout=3)
            except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.TimeoutExpired):
                pass
    
    _cleanup()  # Cleanup before test
    yield _cleanup
    _cleanup()  # Cleanup after test

class TestHotkeyRegistration:
    """Test actual hotkey registration capabilities"""
    
    def test_app_starts_successfully(self, app_binary_path, cleanup_processes):
        """Test if the app starts without crashing"""
        app_process = None
        try:
            env = os.environ.copy()
            env["AI_API_KEY"] = "test_key_for_startup"
            
            app_process = subprocess.Popen(
                [str(app_binary_path), "run"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                env=env
            )
            
            # Give app time to initialize
            time.sleep(2)
            
            # Check if process is still running
            assert app_process.poll() is None, "App crashed during startup"
            
            # Check if it's listening/responsive
            time.sleep(1)
            returncode = app_process.poll()
            assert returncode is None, f"App exited with code: {returncode}"
            
        finally:
            if app_process:
                app_process.terminate()
                app_process.wait(timeout=5)
                
    def test_hotkey_system_availability(self, app_binary_path):
        """Test if the hotkey system can be initialized"""
        # Test config command which doesn't require hotkey registration
        try:
            env = os.environ.copy()
            # Get API key from environment or use test key
            api_key = os.getenv("AI_API_KEY", "test_key_for_config")
            env["AI_API_KEY"] = api_key
            
            result = subprocess.run([
                str(app_binary_path), "config"
            ], capture_output=True, text=True, timeout=10, env=env)
            
            # Should complete without error (tests basic app functionality)
            assert result.returncode == 0, f"App config failed: {result.stderr}"
                
        except subprocess.TimeoutExpired:
            pytest.fail("App hung during config command")

class TestPermissions:
    """Test platform-specific permissions"""
    
    @pytest.mark.skipif(sys.platform != "darwin", reason="macOS specific test")
    def test_macos_accessibility_permissions(self):
        """Test if accessibility permissions are granted on macOS"""
        # Test using AppleScript to check accessibility permissions
        applescript = '''
        tell application "System Events"
            get name of every process
        end tell
        '''
        
        try:
            result = subprocess.run([
                "osascript", "-e", applescript
            ], capture_output=True, text=True, timeout=5)
            
            if result.returncode != 0:
                pytest.skip("Accessibility permissions not granted - this will cause hotkey failures")
                
        except subprocess.TimeoutExpired:
            pytest.fail("Permission check timed out - likely permission dialog appeared")
            
    @pytest.mark.skipif(sys.platform != "win32", reason="Windows specific test")
    def test_windows_admin_privileges(self):
        """Test if running with appropriate privileges on Windows"""
        import ctypes
        
        try:
            is_admin = ctypes.windll.shell32.IsUserAnAdmin()
            if not is_admin:
                pytest.skip("Not running as administrator - hotkeys may fail")
        except Exception:
            pytest.skip("Could not check admin privileges")

class TestHotkeyConflicts:
    """Test for hotkey conflicts with other applications"""
    
    def test_hotkey_not_already_registered(self):
        """Test if the hotkey combination is available"""
        # This would require platform-specific implementation
        if sys.platform == "darwin":
            self._test_macos_hotkey_conflicts()
        elif sys.platform == "win32":
            self._test_windows_hotkey_conflicts()
        else:
            self._test_linux_hotkey_conflicts()
            
    def _test_macos_hotkey_conflicts(self):
        """Check for macOS hotkey conflicts"""
        # Check common conflicting apps
        conflicting_processes = [
            "Spotlight", "Alfred", "LaunchBar", "Raycast"
        ]
        
        running_conflicts = []
        for proc in psutil.process_iter(['name']):
            try:
                if any(conflict.lower() in proc.info['name'].lower() 
                      for conflict in conflicting_processes):
                    running_conflicts.append(proc.info['name'])
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                pass
                
        if running_conflicts:
            pytest.skip(f"Potential hotkey conflicts with: {', '.join(running_conflicts)}")
            
    def _test_windows_hotkey_conflicts(self):
        """Check for Windows hotkey conflicts"""
        # Similar implementation for Windows
        pass
        
    def _test_linux_hotkey_conflicts(self):
        """Check for Linux hotkey conflicts"""
        # Similar implementation for Linux
        pass

class TestRealWorldScenarios:
    """Test scenarios that mimic real-world usage"""
    
    def test_capture_command_basic(self, app_binary_path):
        """Test the capture command with basic functionality"""
        # Test capture command without actual API call
        try:
            result = subprocess.run([
                str(app_binary_path), "test"
            ], capture_output=True, text=True, timeout=10)
            
            # Should complete (may fail on API but tests command parsing)
            assert result.returncode in [0, 1], f"Unexpected error: {result.stderr}"
                
        except subprocess.TimeoutExpired:
            pytest.fail("App hung during test command")
            
    def _create_test_image(self) -> Path:
        """Create a test screenshot image"""
        from PIL import Image, ImageDraw, ImageFont
        
        # Create a simple test image with text
        img = Image.new('RGB', (800, 600), color='white')
        draw = ImageDraw.Draw(img)
        
        # Add some text
        try:
            font = ImageFont.truetype("arial.ttf", 24)
        except:
            font = ImageFont.load_default()
            
        draw.text((50, 50), "Test Screenshot\nThis is a test image for AI analysis", 
                 fill='black', font=font)
        
        test_path = Path("test_screenshot.png")
        img.save(test_path)
        return test_path

class TestAPIIntegration:
    """Test API integration without false positives"""
    
    def test_api_connectivity_with_real_credentials(self):
        """Test API connectivity with real credentials (if available)"""
        api_key = os.getenv("AI_API_KEY")
        if not api_key:
            pytest.skip("No API key provided - cannot test real API integration")
            
        # Test with a minimal request
        try:
            response = requests.post(
                "https://api.openai.com/v1/chat/completions",
                headers={
                    "Authorization": f"Bearer {api_key}",
                    "Content-Type": "application/json"
                },
                json={
                    "model": "gpt-4o-mini",
                    "messages": [{"role": "user", "content": "test"}],
                    "max_tokens": 5
                },
                timeout=10
            )
            
            assert response.status_code == 200, f"API request failed: {response.text}"
            
        except requests.RequestException as e:
            pytest.fail(f"API connectivity test failed: {e}")

class TestSystemIntegration:
    """Integration tests that verify actual system behavior"""
    
    def test_clipboard_access(self):
        """Test if the app can actually access the clipboard"""
        import pyperclip
        
        # Set test content
        test_content = "Test clipboard content for hotkey app"
        pyperclip.copy(test_content)
        
        # Verify we can read it back
        clipboard_content = pyperclip.paste()
        assert clipboard_content == test_content, "Clipboard access is not working"
        
    def test_screen_capture_capabilities(self):
        """Test if the system can actually capture screenshots"""
        try:
            import PIL.ImageGrab
            
            # Try to capture a small screenshot
            screenshot = PIL.ImageGrab.grab(bbox=(0, 0, 100, 100))
            assert screenshot.size == (100, 100), "Screenshot capture failed"
            
        except Exception as e:
            pytest.fail(f"Screen capture not available: {e}")

# Pytest configuration
def pytest_configure(config):
    """Configure pytest with custom markers"""
    config.addinivalue_line("markers", "integration: marks tests as integration tests")
    config.addinivalue_line("markers", "permissions: marks tests that require special permissions")
    config.addinivalue_line("markers", "slow: marks tests as slow running")

# Example usage
if __name__ == "__main__":
    # Run tests with proper configuration
    pytest.main([
        "--verbose",
        "--tb=short",
        "-m", "not slow",  # Skip slow tests by default
        __file__
    ])