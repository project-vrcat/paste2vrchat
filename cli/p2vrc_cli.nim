import os
import urlly
import winim/lean
import winregistry

proc SwitchToWindow(class: LPCWSTR, title: LPCWSTR): bool =
  let hwnd = FindWindow(class, title)
  if hwnd == 0: return false
  return SetForegroundWindow(hwnd)

# https://docs.microsoft.com/windows/win32/inputdev/virtual-key-codes
proc Paste() =
  keybd_event(VK_LCONTROL, 0, 0, 0)
  keybd_event(0x56, 0, 0, 0)
  keybd_event(0x56, 0, KEYEVENTF_KEYUP, 0)
  keybd_event(VK_LCONTROL, 0, KEYEVENTF_KEYUP, 0)
  keybd_event(VK_RETURN, 0, 0, 0)
  keybd_event(VK_RETURN, 0, KEYEVENTF_KEYUP, 0)

proc SetClipboard(text: string) =
  if OpenClipboard(0) != TRUE: return
  let size = len(text) + 1
  let data = GlobalAlloc(GMEM_MOVEABLE, size.int32)
  if data == 0: return
  let mem = GlobalLock(data)
  if mem == nil: return
  copyMem(mem, text.cstring, size)
  discard GlobalUnlock(data)
  discard EmptyClipboard()
  discard SetClipboardData(CF_TEXT,data)
  discard CloseClipboard()

proc RegisterUrlScheme() =
  const 
    regPath = "HKEY_CURRENT_USER\\Software\\Classes\\p2vrc"
    commandPath = regPath & "\\shell\\open\\command"
  var
    h: RegHandle
    commandH: RegHandle
    command: string = "\"" & getAppFilename() & "\" --open-url \"%1\""
  try:
    h = open(regPath, samAll)
  except:
    h = create(regPath, samAll)
  writeString(h, "", "URL:p2vrc")
  writeString(h, "URL Protocol", "")
  close(h)

  try:
    commandH = open(commandPath, samAll)
  except:
    commandH = create(commandPath, samAll)
  writeString(commandH, "", command)
  close(commandH)

proc main() =
  var text:string
  if paramCount() == 1 and paramStr(1) == "--setup":
    RegisterUrlScheme()
    MessageBox(0, "Setup complete", "Paste2VRChat Setup", MB_OK)
    quit(QuitSuccess)
  elif paramCount() == 1:
    text = paramStr(1)
  elif paramCount() == 2 and paramStr(1) == "--open-url":
    let url = paramStr(2)
    let res = parseUrl(url)
    if res.scheme != "p2vrc": return
    case res.hostname:
      of "paste":
        text = res.query["text"]
      of "check":
        MessageBox(0, "Success", "Paste2VRChat Check", MB_OK)
        quit(QuitSuccess)

  if text == "": return
  if not SwitchToWindow(nil, "VRChat"): return
  SetClipboard(text)
  Paste()
  quit(QuitSuccess)

main()
quit(QuitFailure)
