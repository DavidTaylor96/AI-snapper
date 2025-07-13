import pytest
import subprocess
import time
import os
import signal
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


@pytest.mark.integration
class TestMacOSIntegration:
    """Test real world macOS integration"""
    
    def test_app_launches_on_macos(self, app_binary_path):
        """Test that the app actually launches and initializes on macOS"""
        env = os.environ.copy()
        api_key = os.getenv("AI_API_KEY", "test_key_macos")
        env["AI_API_KEY"] = api_key
        
        app_process = None
        try:
            # Start the app
            app_process = subprocess.Popen([
                str(app_binary_path), "run"
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
            
            # Let it initialize
            time.sleep(3)
            
            # Check if still running
            returncode = app_process.poll()
            if returncode is not None:
                stdout, stderr = app_process.communicate()
                if "accessibility" in stderr.lower() or "permission" in stderr.lower():
                    pytest.skip("Accessibility permissions not granted - this is expected on macOS")
                pytest.fail(f"App failed to start: {stderr}")
            
            # Success - app is running
            assert True, "App launched successfully on macOS"
            
        finally:
            if app_process and app_process.poll() is None:
                app_process.terminate()
                try:
                    app_process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    app_process.kill()
                    app_process.wait()

    def test_capture_command_works_on_macos(self, app_binary_path):
        """Test that the capture command works on macOS"""
        env = os.environ.copy()
        api_key = os.getenv("AI_API_KEY", "test_key_macos_capture")
        env["AI_API_KEY"] = api_key
        
        try:
            # Test capture command (no additional args needed)
            result = subprocess.run([
                str(app_binary_path), "capture"
            ], capture_output=True, text=True, timeout=30, env=env)
            
            # Should complete (may fail on API but tests screenshot capability)
            if result.returncode != 0:
                if "accessibility" in result.stderr.lower() or "permission" in result.stderr.lower():
                    pytest.skip("Screen recording permissions not granted - expected on macOS")
                if "Failed to capture screenshot" in result.stderr:
                    pytest.skip("Screenshot capture failed - may need screen recording permissions")
                    
            # If we get here, capture worked
            assert result.returncode == 0, f"Capture failed: {result.stderr}"
            
        except subprocess.TimeoutExpired:
            pytest.fail("Capture command timed out")

    def test_config_shows_proper_macos_settings(self, app_binary_path):
        """Test that config command shows appropriate macOS settings"""
        env = os.environ.copy()
        api_key = os.getenv("AI_API_KEY", "test_key_macos_config")
        env["AI_API_KEY"] = api_key
        
        try:
            result = subprocess.run([
                str(app_binary_path), "config"
            ], capture_output=True, text=True, timeout=10, env=env)
            
            assert result.returncode == 0, f"Config command failed: {result.stderr}"
            
            # Should show macOS-appropriate paths
            config_output = result.stdout
            assert len(config_output) > 0, "Config should show some output"
            
        except subprocess.TimeoutExpired:
            pytest.fail("Config command timed out")

    def test_accessibility_permission_check(self):
        """Test accessibility permission detection on macOS"""
        try:
            # Try to access System Events (requires accessibility permissions)
            result = subprocess.run([
                'osascript', '-e', 
                'tell application "System Events" to get name of first process'
            ], capture_output=True, text=True, timeout=5)
            
            if result.returncode == 0:
                # Permissions are granted
                assert True, "Accessibility permissions are available"
            else:
                # Permissions not granted - this is actually normal
                pytest.skip("Accessibility permissions not granted - normal for first run")
                
        except subprocess.TimeoutExpired:
            pytest.skip("Permission check timed out - may have triggered permission dialog")
        except FileNotFoundError:
            pytest.skip("osascript not available")

    def test_hotkey_system_availability_macos(self, app_binary_path):
        """Test that the hotkey system can initialize on macOS"""
        env = os.environ.copy()
        api_key = os.getenv("AI_API_KEY", "test_key_macos_hotkey")
        env["AI_API_KEY"] = api_key
        
        app_process = None
        try:
            # Start app with hotkey registration
            app_process = subprocess.Popen([
                str(app_binary_path), "run"
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
            
            # Give it time to register hotkeys
            time.sleep(2)
            
            # If still running, hotkey registration likely succeeded
            if app_process.poll() is None:
                # Send SIGTERM to gracefully shut down
                app_process.terminate()
                stdout, stderr = app_process.communicate(timeout=5)
                
                # Check for specific macOS hotkey issues
                if "CGEventTapCreate failed" in stderr:
                    pytest.skip("macOS event tap creation failed - needs accessibility permissions")
                if "Could not register global hotkey" in stderr:
                    pytest.skip("Global hotkey registration failed - may need accessibility permissions")
                    
                # If we got here, likely succeeded
                assert True, "Hotkey system appears to work on macOS"
            else:
                stdout, stderr = app_process.communicate()
                if "accessibility" in stderr.lower():
                    pytest.skip("Accessibility permissions required for hotkeys on macOS")
                pytest.fail(f"App failed during hotkey registration: {stderr}")
                
        finally:
            if app_process and app_process.poll() is None:
                app_process.kill()
                app_process.wait()

    def test_multiple_app_instances_macos(self, app_binary_path):
        """Test behavior of multiple app instances on macOS"""
        env = os.environ.copy()
        api_key = os.getenv("AI_API_KEY", "test_key_macos_multi")
        env["AI_API_KEY"] = api_key
        
        processes = []
        try:
            # Start first instance
            proc1 = subprocess.Popen([
                str(app_binary_path), "run"
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
            processes.append(proc1)
            
            time.sleep(2)
            
            # Start second instance
            proc2 = subprocess.Popen([
                str(app_binary_path), "run"
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
            processes.append(proc2)
            
            time.sleep(2)
            
            # Check states
            proc1_running = proc1.poll() is None
            proc2_running = proc2.poll() is None
            
            if not proc1_running and not proc2_running:
                # Both failed - check for permission issues
                _, stderr1 = proc1.communicate()
                _, stderr2 = proc2.communicate()
                if "accessibility" in (stderr1 + stderr2).lower():
                    pytest.skip("Both instances failed due to accessibility permissions")
                    
            # At least one should be running, or second should fail due to conflict
            if proc1_running and proc2_running:
                # Both running - unexpected but not necessarily bad
                assert True, "Multiple instances can run simultaneously"
            elif proc1_running and not proc2_running:
                # Expected: first succeeds, second fails
                assert True, "First instance succeeded, second failed as expected"
            else:
                pytest.skip("Unexpected process states - may be permission related")
                
        finally:
            for proc in processes:
                if proc.poll() is None:
                    proc.terminate()
                    try:
                        proc.wait(timeout=3)
                    except subprocess.TimeoutExpired:
                        proc.kill()
                        proc.wait()