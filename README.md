# **printctl**

A reverse proxy for serial devices over the network, purpose-built for managing 3D printing clusters at scale.

Unlike traditional solutions like OctoPrint, **printctl** is designed for effortless deployment with a **zero-config** philosophy; just fire it up, and it works.
Built in **Rust**, it delivers rock-solid performance, enabling seamless remote management of 3D printers.

## **✨ Features**

-   🚀 **Zero-Config Networking** – Uses multicast DNS (mDNS) for automatic discovery. No setup, no hassle.
-   ⚡ **gRPC-Powered Communication** – Blazing-fast, efficient, and built for modern distributed systems.
-   📡 **Live Printer Logs** – Stream real-time serial output for instant debugging and monitoring.
-   🛠 **Direct Serial Commands** – Send commands straight to your printers, no middleman needed.
-   📂 **GCODE Execution** – Upload and execute print jobs with ease.

## **🖧 Client/Server Architecture**

-   **Client**: Auto-discovers available **printctl** servers and handles communication.
-   **Server**: Manages serial devices, tracks printer availability, and executes jobs.

Built for power users, hackers, and makers—**printctl** turns your 3D printing fleet into a seamlessly connected, remotely controlled powerhouse.
