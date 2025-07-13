#!/usr/bin/env python3
"""
Automation test script for AI Screenshot Analyzer

This script:
1. Builds and starts the AI screenshot analyzer application
2. Simulates the Cmd+Shift+S hotkey to trigger a screenshot
3. Waits for and verifies the AI response
4. Cleans up processes and validates the test results

Requirements:
- Python 3.7+
- pip install psutil pynput

Usage:
    python3 scripts/automation_test.py [--provider openai|claude] [--timeout 60]
"""

import argparse
import os
import signal
import subprocess
import sys
import time
import threading
from pathlib import Path
from pynput import keyboard
import psutil

class AutomationTest:
    def __init__(self, provider="openai", timeout=60):
        self.provider = provider
        self.timeout = timeout
        self.project_dir = Path(__file__).parent.parent
        self.app_process = None
        self.output_lines = []
        self.ai_response_received = False
        self.test_passed = False
        self.start_time = None
        
    def log(self, message):
        """Log message with timestamp"""
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] {message}")
        
    def build_application(self):
        """Build the application in release mode"""
        self.log("🔨 Building application...")
        try:
            result = subprocess.run(
                ["cargo", "build", "--release"],
                cwd=self.project_dir,
                capture_output=True,
                text=True,
                timeout=120
            )
            if result.returncode != 0:
                self.log(f"❌ Build failed: {result.stderr}")
                return False
            self.log("✅ Build successful")
            return True
        except subprocess.TimeoutExpired:
            self.log("❌ Build timed out after 2 minutes")
            return False
        except Exception as e:
            self.log(f"❌ Build error: {e}")
            return False
            
    def start_application(self):
        """Start the AI screenshot analyzer application"""
        self.log(f"🚀 Starting application with provider: {self.provider}")
        
        # Set environment for the application
        env = os.environ.copy()
        
        try:
            # Start the application process
            self.app_process = subprocess.Popen(
                ["cargo", "run", "--release", "--", "--provider", self.provider, "run"],
                cwd=self.project_dir,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                universal_newlines=True,
                env=env
            )
            
            # Start a thread to read output
            self.output_thread = threading.Thread(target=self._read_output, daemon=True)
            self.output_thread.start()
            
            # Wait for app to initialize
            self.log("⏳ Waiting for application to initialize...")
            time.sleep(5)
            
            # Check if process is still running
            if self.app_process.poll() is not None:
                self.log("❌ Application failed to start")
                return False
                
            self.log("✅ Application started successfully")
            return True
            
        except Exception as e:
            self.log(f"❌ Failed to start application: {e}")
            return False
            
    def _read_output(self):
        """Read application output in a separate thread"""
        try:
            for line in iter(self.app_process.stdout.readline, ''):
                if line:
                    line = line.strip()
                    self.output_lines.append(line)
                    self.log(f"APP: {line}")
                    
                    # Check for AI response indicators
                    if "💡 Analysis Result:" in line or "✅" in line:
                        self.ai_response_received = True
                        self.log("🎉 AI response detected!")
                        
                    # Check for error indicators
                    if "❌" in line and "failed" in line.lower():
                        self.log(f"🚨 Error detected: {line}")
                        
        except Exception as e:
            self.log(f"⚠️ Error reading output: {e}")
            
    def simulate_hotkey(self):
        """Simulate the Cmd+Shift+S hotkey"""
        self.log("⌨️ Simulating Cmd+Shift+S hotkey...")
        
        try:
            # Give the application a moment to be ready
            time.sleep(2)
            
            # Try both pynput and AppleScript approaches
            success = False
            
            # Method 1: Try pynput
            try:
                self.log("🔄 Trying pynput method...")
                from pynput.keyboard import Key, Listener
                from pynput import keyboard
                
                # Create a controller
                kb = keyboard.Controller()
                
                # Press and release the hotkey combination
                with kb.pressed(Key.cmd, Key.shift):
                    kb.press('s')
                    kb.release('s')
                    
                time.sleep(0.5)
                
                # Try a second time
                with kb.pressed(Key.cmd, Key.shift):
                    kb.press('s')
                    kb.release('s')
                    
                self.log("✅ pynput hotkey simulation completed")
                success = True
                
            except Exception as e:
                self.log(f"⚠️ pynput method failed: {e}")
            
            # Method 2: Try AppleScript with CGEvent as backup
            if not success:
                self.log("🔄 Trying AppleScript with CGEvent method...")
                applescript = '''
                tell application "System Events"
                    delay 0.3
                    key code 1 using {command down, shift down}
                    delay 0.1
                    key code 1 using {command down, shift down}
                    delay 0.1
                    key code 1 using {command down, shift down}
                    delay 0.1
                    key code 1 using {command down, shift down}
                    delay 0.1
                    key code 1 using {command down, shift down}
                end tell
                '''
                
                result = subprocess.run(
                    ["osascript", "-e", applescript],
                    capture_output=True,
                    text=True,
                    timeout=15
                )
                
                if result.returncode == 0:
                    self.log("✅ AppleScript hotkey simulation completed")
                    success = True
                else:
                    self.log(f"❌ AppleScript simulation failed: {result.stderr}")
            
            # Method 3: Try direct keystroke as final backup
            if not success:
                self.log("🔄 Trying direct keystroke method...")
                applescript = '''
                tell application "System Events"
                    delay 0.5
                    keystroke "s" using {command down, shift down}
                    delay 0.2
                    keystroke "s" using {command down, shift down}
                    delay 0.2
                    keystroke "s" using {command down, shift down}
                end tell
                '''
                
                result = subprocess.run(
                    ["osascript", "-e", applescript],
                    capture_output=True,
                    text=True,
                    timeout=15
                )
                
                if result.returncode == 0:
                    self.log("✅ Direct keystroke simulation completed")
                    success = True
                else:
                    self.log(f"❌ Direct keystroke simulation failed: {result.stderr}")
            
            if success:
                self.start_time = time.time()
                return True
            else:
                self.log("❌ All hotkey simulation methods failed")
                return False
                
        except Exception as e:
            self.log(f"❌ Hotkey simulation error: {e}")
            return False
            
    def wait_for_ai_response(self):
        """Wait for AI response with timeout"""
        self.log(f"⏳ Waiting for AI response (timeout: {self.timeout}s)...")
        
        start_wait = time.time()
        while time.time() - start_wait < self.timeout:
            if self.ai_response_received:
                elapsed = time.time() - self.start_time if self.start_time else 0
                self.log(f"🎉 AI response received in {elapsed:.2f} seconds!")
                return True
                
            # Check if application is still running
            if self.app_process and self.app_process.poll() is not None:
                self.log("❌ Application process terminated unexpectedly")
                return False
                
            time.sleep(1)
            
        self.log(f"⏰ Timeout reached ({self.timeout}s) - no AI response")
        return False
        
    def cleanup(self):
        """Clean up processes and resources"""
        self.log("🧹 Cleaning up...")
        
        if self.app_process:
            try:
                # Send SIGTERM first
                self.app_process.terminate()
                
                # Wait a bit for graceful shutdown
                try:
                    self.app_process.wait(timeout=5)
                    self.log("✅ Application terminated gracefully")
                except subprocess.TimeoutExpired:
                    # Force kill if it doesn't terminate gracefully
                    self.log("⚠️ Force killing application...")
                    self.app_process.kill()
                    self.app_process.wait()
                    
            except Exception as e:
                self.log(f"⚠️ Error during cleanup: {e}")
                
        # Clean up any remaining cargo processes
        try:
            for proc in psutil.process_iter(['pid', 'name', 'cmdline']):
                try:
                    if (proc.info['name'] == 'cargo' or 
                        'ai-screenshot-analyzer' in ' '.join(proc.info['cmdline'] or [])):
                        proc.terminate()
                        self.log(f"🧹 Terminated process: {proc.info['name']} (PID: {proc.info['pid']})")
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    pass
        except Exception as e:
            self.log(f"⚠️ Error cleaning up processes: {e}")
            
    def validate_results(self):
        """Validate test results"""
        self.log("📊 Validating test results...")
        
        # Check if we got AI response
        if not self.ai_response_received:
            self.log("❌ No AI response received")
            return False
            
        # Check output for success indicators
        success_indicators = ["💡 Analysis Result:", "✅"]
        has_success = any(indicator in ' '.join(self.output_lines) for indicator in success_indicators)
        
        if not has_success:
            self.log("❌ No success indicators found in output")
            return False
            
        # Check for error indicators
        error_indicators = ["❌", "failed", "error"]
        has_errors = any(indicator.lower() in ' '.join(self.output_lines).lower() 
                        for indicator in error_indicators)
        
        if has_errors:
            self.log("⚠️ Warning: Error indicators found in output")
            
        self.log("✅ Test validation successful")
        return True
        
    def run_test(self):
        """Run the complete automation test"""
        self.log("🧪 Starting AI Screenshot Analyzer Automation Test")
        self.log("=" * 60)
        
        try:
            # Step 1: Build application
            if not self.build_application():
                return False
                
            # Step 2: Start application
            if not self.start_application():
                return False
                
            # Step 3: Simulate hotkey
            if not self.simulate_hotkey():
                return False
                
            # Step 4: Wait for AI response
            if not self.wait_for_ai_response():
                return False
                
            # Step 5: Validate results
            if not self.validate_results():
                return False
                
            self.test_passed = True
            self.log("🎉 AUTOMATION TEST PASSED!")
            return True
            
        except KeyboardInterrupt:
            self.log("⚠️ Test interrupted by user")
            return False
        except Exception as e:
            self.log(f"❌ Unexpected error: {e}")
            return False
        finally:
            self.cleanup()
            
    def print_summary(self):
        """Print test summary"""
        self.log("=" * 60)
        self.log("📋 TEST SUMMARY")
        self.log("=" * 60)
        self.log(f"Provider: {self.provider}")
        self.log(f"Timeout: {self.timeout}s")
        self.log(f"AI Response Received: {'✅' if self.ai_response_received else '❌'}")
        self.log(f"Test Result: {'✅ PASSED' if self.test_passed else '❌ FAILED'}")
        
        if self.output_lines:
            self.log("\n📝 Application Output (last 10 lines):")
            for line in self.output_lines[-10:]:
                self.log(f"  {line}")


def main():
    parser = argparse.ArgumentParser(description="AI Screenshot Analyzer Automation Test")
    parser.add_argument("--provider", choices=["openai", "claude", "gemini"], 
                       default="openai", help="AI provider to use")
    parser.add_argument("--timeout", type=int, default=60, 
                       help="Timeout in seconds for AI response")
    parser.add_argument("--verbose", "-v", action="store_true", 
                       help="Enable verbose output")
    
    args = parser.parse_args()
    
    # Check requirements
    try:
        import pynput
        import psutil
    except ImportError as e:
        print(f"❌ Missing required dependency: {e}")
        print("Install with: pip install psutil pynput")
        sys.exit(1)
        
    # Check if running on macOS
    if sys.platform != "darwin":
        print("❌ This automation test is designed for macOS only")
        sys.exit(1)
        
    # Check for API key
    if not os.getenv("AI_API_KEY"):
        print("❌ AI_API_KEY environment variable not set")
        print("Set it in .env file or export AI_API_KEY=your_key")
        sys.exit(1)
        
    # Run the test
    test = AutomationTest(provider=args.provider, timeout=args.timeout)
    success = test.run_test()
    test.print_summary()
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()