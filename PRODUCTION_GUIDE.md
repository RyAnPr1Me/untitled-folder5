# Production Deployment Guide

## Overview

This guide covers deploying the Advanced Network Packet Sniffer in production environments, including enterprise deployment, security considerations, and maintenance procedures.

## Deployment Options

### 1. Linux Package Distribution

#### Using the Installation Script
```bash
# Extract distribution package
tar -xzf packet_sniffer_v1.0.0.tar.gz
cd packet_sniffer_v1.0.0

# Run installation script
sudo ./install_linux.sh
```

#### Manual Installation
```bash
# Copy binary to system location
sudo cp packet_sniffer_linux /usr/local/bin/packet_sniffer
sudo chmod +x /usr/local/bin/packet_sniffer

# Create configuration directory
sudo mkdir -p /etc/packet_sniffer
sudo cp config/default_config.json /etc/packet_sniffer/

# Create system user (optional, for daemon mode)
sudo useradd -r -s /bin/false packet_sniffer
```

### 2. Windows Enterprise Deployment

#### MSI Package (Recommended for Enterprise)
- Use Windows Installer XML (WiX) to create MSI package
- Deploy via Group Policy or SCCM
- Includes automatic Npcap dependency management

#### Inno Setup Installer
- Use provided installer.iss script
- Suitable for individual installations
- Requires administrator privileges

### 3. Container Deployment

#### Docker Image
```dockerfile
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    libpnet-dev \
    && rm -rf /var/lib/apt/lists/*

COPY packet_sniffer_linux /usr/local/bin/packet_sniffer
COPY config/ /etc/packet_sniffer/

# Note: Requires --privileged or --cap-add=NET_RAW
ENTRYPOINT ["packet_sniffer"]
```

#### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: packet-sniffer
spec:
  template:
    spec:
      hostNetwork: true
      containers:
      - name: packet-sniffer
        image: packet-sniffer:1.0.0
        securityContext:
          privileged: true
          capabilities:
            add: ["NET_RAW", "NET_ADMIN"]
```

## Security Considerations

### 1. Privilege Requirements
- **Linux**: Requires CAP_NET_RAW capability or root access
- **Windows**: Requires Administrator privileges
- **Mitigation**: Use sudo/runas for specific operations only

### 2. Data Protection
- **Configuration**: Store sensitive configs in protected directories
- **Exports**: Implement encryption for exported packet data
- **Logs**: Rotate and secure log files regularly

### 3. Network Access Control
```json
{
  "security": {
    "allowed_interfaces": ["eth0", "wlan0"],
    "restricted_protocols": ["ssh", "https"],
    "max_capture_duration": 3600,
    "auto_anonymize": true
  }
}
```

### 4. Compliance Considerations
- **Data Retention**: Implement automatic cleanup policies
- **Audit Logging**: Log all packet capture activities
- **Access Control**: Implement role-based access controls

## Production Configuration

### 1. System Service Configuration

#### Linux Systemd Service
```ini
[Unit]
Description=Advanced Network Packet Sniffer
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/packet_sniffer --config /etc/packet_sniffer/production.json
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

#### Windows Service
- Use NSSM (Non-Sucking Service Manager) or similar
- Configure automatic startup
- Set appropriate service account with necessary privileges

### 2. Production Configuration Template
```json
{
  "logging": {
    "level": "info",
    "file": "/var/log/packet_sniffer/application.log",
    "enable_console": false,
    "enable_file": true,
    "max_file_size": "100MB",
    "max_backup_files": 10
  },
  "performance": {
    "buffer_size": 8192,
    "max_packets_per_second": 10000,
    "dashboard_refresh_rate": 5000
  },
  "security": {
    "allowed_interfaces": ["eth0"],
    "max_capture_duration": 7200,
    "auto_export_interval": 3600,
    "anonymize_ips": true
  },
  "export": {
    "default_format": "json",
    "default_directory": "/var/lib/packet_sniffer/exports",
    "auto_backup": true,
    "compression": true,
    "encryption": true
  }
}
```

## Monitoring and Maintenance

### 1. Health Checks
```bash
#!/bin/bash
# health_check.sh
if pgrep -f packet_sniffer > /dev/null; then
    echo "packet_sniffer is running"
    exit 0
else
    echo "packet_sniffer is not running"
    exit 1
fi
```

### 2. Log Rotation
```bash
# /etc/logrotate.d/packet_sniffer
/var/log/packet_sniffer/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    postrotate
        systemctl reload packet_sniffer
    endscript
}
```

### 3. Monitoring Integration

#### Prometheus Metrics
- Implement metrics endpoint for packet counts
- Monitor interface status and capture rates
- Alert on anomalous traffic patterns

#### ELK Stack Integration
- Export logs to Elasticsearch
- Create Kibana dashboards for traffic analysis
- Set up alerting rules in Watcher

## Performance Tuning

### 1. System Optimizations
```bash
# Increase network buffer sizes
echo 'net.core.rmem_max = 134217728' >> /etc/sysctl.conf
echo 'net.core.rmem_default = 134217728' >> /etc/sysctl.conf

# Disable unnecessary services
systemctl disable cups
systemctl disable bluetooth

# Set CPU affinity for packet processing
taskset -c 0,1 packet_sniffer --interface eth0
```

### 2. Application Tuning
- Adjust buffer sizes based on network load
- Implement packet filtering at kernel level
- Use multiple worker threads for high-throughput scenarios

## Troubleshooting Guide

### Common Issues

1. **Permission Denied**
   - Verify user has required privileges
   - Check SELinux/AppArmor policies
   - Ensure proper group memberships

2. **Interface Not Found**
   - Verify interface exists and is up
   - Check interface naming conventions
   - Ensure driver compatibility

3. **High CPU Usage**
   - Implement packet filtering
   - Reduce capture rate
   - Check for memory leaks

4. **Storage Issues**
   - Implement automatic cleanup
   - Monitor disk space
   - Compress older exports

### Debug Mode
```bash
# Enable detailed logging
RUST_LOG=debug packet_sniffer --interface eth0 --verbose
```

## Scaling Considerations

### Horizontal Scaling
- Deploy multiple instances for different network segments
- Use load balancers for management interfaces
- Implement centralized configuration management

### Vertical Scaling
- Increase buffer sizes for high-throughput networks
- Use faster storage for export operations
- Consider GPU acceleration for packet processing

## Backup and Recovery

### Configuration Backup
```bash
#!/bin/bash
# backup_config.sh
tar -czf packet_sniffer_config_$(date +%Y%m%d).tar.gz \
    /etc/packet_sniffer/ \
    /var/lib/packet_sniffer/
```

### Data Recovery
- Implement export verification
- Maintain checksums for critical data
- Test restore procedures regularly

## Support and Maintenance

### Update Procedures
1. Test updates in staging environment
2. Backup current configuration and data
3. Deploy during maintenance windows
4. Verify functionality post-update

### Contact Information
- Technical Support: [support email]
- Documentation: [documentation URL]
- Issue Tracking: [GitHub/Jira URL]