# D-Bus API Reference

## Asusd (xyz.ljones.Asusd)

### Path: `/xyz/ljones/Platform`

#### Properties

- `profile` (string): Current power profile (quiet/balanced/performance)
- `charge_control_end_threshold` (uint8): Battery charge limit (0-100)

### Methods

TODO: Add complete method documentation

## Supergfxd (org.asuslinux.Supergfxd)

### Path: `/org/asuslinux/Supergfxd`

TODO: Add complete API documentation

## Implementation Notes

### Power Profiles

The application supports three power profiles:

- **Quiet**: Reduced performance, lower fan noise, lower power consumption
- **Balanced**: Default performance profile
- **Performance**: Maximum performance, higher fan noise

Switching profiles changes:
- CPU power limits
- Fan curves
- System performance settings

### Charge Limits

Battery charge limit can be set from 50-100%. Setting to 100 disables the limit.

Recommended limits:
- **Daily use**: 80-90%
- **Always plugged in**: 90-100%
- **Battery storage**: 50-60%

### Hardware Monitoring

The application polls hardware metrics every second:

- **CPU Temperature**: Current CPU core temperature in Celsius
- **GPU Temperature**: Current GPU temperature (if dedicated GPU is active)
- **CPU Fan**: CPU fan speed in RPM and percentage
- **GPU Fan**: GPU fan speed (if active)
- **Power Draw**: Total system power consumption in Watts

### GPU Modes

Three GPU modes are supported (via supergfxctl):

- **Integrated**: Uses only integrated GPU (iGPU), saves power
- **Hybrid**: Automatic switching based on workload
- **Dedicated**: Uses only dedicated GPU (dGPU), maximum performance

**Note**: GPU mode switching may require user authentication and can take several seconds.
