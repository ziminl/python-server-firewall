import subprocess

def add_anydesk_firewall_rule():
    command = "New-NetFirewallRule -DisplayName 'Allow AnyDesk' -Direction Inbound -Action Allow -Protocol TCP -LocalPort 80,443,6568,50001-50003 -Description 'Allow AnyDesk connections'"
    subprocess.run(["powershell", "-Command", command], check=True)

add_anydesk_firewall_rule()
