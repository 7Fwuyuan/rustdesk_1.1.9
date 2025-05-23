lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Trạng thái hiện tại"),
        ("Your Desktop", "Desktop của bạn"),
        ("desk_tip", "Desktop của bạn có thể đuợc truy cập bằng ID và mật khẩu này."),
        ("Password", "Mật khẩu"),
        ("Ready", "Sẵn sàng"),
        ("Established", "Đã đuợc thiết lập"),
        ("connecting_status", "Đang kết nối đến mạng lưới RustDesk..."),
        ("Enable Service", "Bật dịch vụ"),
        ("Start Service", "Bắt đầu dịch vụ"),
        ("Service is running", "Dịch vụ hiện đang chạy"),
        ("Service is not running", "Dịch vụ hiện đang dừng"),
        ("not_ready_status", "Hiện chưa sẵn sàng. Hãy kiểm tra kết nối của bạn"),
        ("Control Remote Desktop", "Điều khiển Desktop Từ Xa"),
        ("Transfer File", "Truyền Tệp Tin"),
        ("Connect", "Kết nối"),
        ("Recent Sessions", "Các session gần đây"),
        ("Address Book", "Quyển địa chỉ"),
        ("Confirmation", "Xác nhận"),
        ("TCP Tunneling", "TCP Tunneling"),
        ("Remove", "Loại bỏ"),
        ("Refresh random password", "Làm mới mật khẩu ngẫu nhiên"),
        ("Set your own password", "Đặt mật khẩu riêng"),
        ("Enable Keyboard/Mouse", "Cho phép sử dụng bàn phím/chuột"),
        ("Enable Clipboard", "Cho phép sử dụng clipboard"),
        ("Enable File Transfer", "Cho phép truyền tệp tin"),
        ("Enable TCP Tunneling", "Cho phép TCP Tunneling"),
        ("IP Whitelisting", "Cho phép IP"),
        ("ID/Relay Server", "Máy chủ ID/Relay"),
        ("Import Server Config", "Nhập cấu hình máy chủ"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "Nhập cấu hình máy chủ thành công"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Cấu hình máy chủ không hợp lệ"),
        ("Clipboard is empty", "Khay nhớ tạm trống"),
        ("Stop service", "Dừng dịch vụ"),
        ("Change ID", "Thay đổi ID"),
        ("Website", "Trang web"),
        ("About", "About"),
        ("Mute", "Tắt tiếng"),
        ("Audio Input", "Đầu vào âm thanh"),
        ("Enhancements", "Các tiện itchs"),
        ("Hardware Codec", "Codec phần cứng"),
        ("Adaptive Bitrate", "Adaptive Bitrate"),
        ("ID Server", "Máy chủ ID"),
        ("Relay Server", "Máy chủ Relay"),
        ("API Server", "Máy chủ API"),
        ("invalid_http", "phải bắt đầu bằng http:// hoặc https://"),
        ("Invalid IP", "IP không hợp lệ"),
        ("id_change_tip", "Các kí tự đuợc phép là: từ a-z, A-Z, 0-9 và _ (dấu gạch dưới). Kí tự đầu tiên phải bắt đầu từ a-z, A-Z. Độ dài kí tự từ 6 đến 16"),
        ("Invalid format", "Định dạng không hợp lệnh"),
        ("server_not_support", "Chưa đuợc hỗ trợ bới server"),
        ("Not available", "Chưa có mặt"),
        ("Too frequent", "Quá thường xuyên"),
        ("Cancel", "Hủy"),
        ("Skip", "Bỏ qua"),
        ("Close", "Đóng"),
        ("Retry", "Thử lại"),
        ("OK", "OK"),
        ("Password Required", "Yêu cầu mật khẩu"),
        ("Please enter your password", "Mời nhập mật khẩu"),
        ("Remember password", "Nhớ mật khẩu"),
        ("Wrong Password", "Sai mật khẩu"),
        ("Do you want to enter again?", "Bạn có muốn nhập lại không"),
        ("Connection Error", "Kết nối bị lỗi"),
        ("Error", "Lỗi"),
        ("Reset by the peer", "Đựoc cài đặt lại với peer"),
        ("Connecting...", "Đang kết nối..."),
        ("Connection in progress. Please wait.", "Đang kết nối. Xin chờ."),
        ("Please try 1 minute later", "Hãy thử lại sau 1 phút"),
        ("Login Error", "Đăng nhập bị lỗi"),
        ("Successful", "Thành công"),
        ("Connected, waiting for image...", "Đã kết nối, đang đợi hình ảnh..."),
        ("Name", "Tên"),
        ("Type", "Loại"),
        ("Modified", "Chỉnh sửa"),
        ("Size", "Kích cỡ"),
        ("Show Hidden Files", "Hiển thị tệp tin bị ẩn"),
        ("Receive", "Nhận"),
        ("Send", "Gửi"),
        ("Refresh File", "Làm mới tệp tin"),
        ("Local", "Cục bộ"),
        ("Remote", "Từ xa"),
        ("Remote Computer", "Máy tính từ xa"),
        ("Local Computer", "Máy tính cục bộ"),
        ("Confirm Delete", "Xác nhận xóa"),
        ("Delete", "Xóa"),
        ("Properties", "Thuộc tính"),
        ("Multi Select", "Chọn nhiều"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "Thư mục rỗng"),
        ("Not an empty directory", "Không phải thư mục rỗng"),
        ("Are you sure you want to delete this file?", "Bạn chắc bạn có muốn xóa tệp tin này không?"),
        ("Are you sure you want to delete this empty directory?", "Bạn chắc bạn có muốn xóa thư mục rỗng này không?"),
        ("Are you sure you want to delete the file of this directory?", "Bạn chắc bạn có muốn xóa những tệp tin trong thư mục này không?"),
        ("Do this for all conflicts", "Xác nhận đối với tất cả các trùng lặp"),
        ("This is irreversible!", "Không thể hoàn tác!"),
        ("Deleting", "Đang xóa"),
        ("files", "các tệp tin"),
        ("Waiting", "Đang chờ"),
        ("Finished", "Hoàn thành"),
        ("Speed", "Tốc độ"),
        ("Custom Image Quality", "Chất lượng hình ảnh"),
        ("Privacy mode", "Chế độ riêng tư"),
        ("Block user input", "Chặn các tương tác từ người dùng"),
        ("Unblock user input", "Hủy chặn các tương tác từ người dùng"),
        ("Adjust Window", "Điều chỉnh cửa sổ"),
        ("Original", "Gốc"),
        ("Shrink", "Thu nhỏ"),
        ("Stretch", "Kéo dãn"),
        ("Scrollbar", "Thanh cuộn"),
        ("ScrollAuto", "Tự động cuộn"),
        ("Good image quality", "Chất lượng hình ảnh tốt"),
        ("Balanced", "Cân bằng"),
        ("Optimize reaction time", "Thời gian phản ứng tối ưu"),
        ("Custom", ""),
        ("Show remote cursor", "Hiển thị con trỏ từ máy từ xa"),
        ("Show quality monitor", "Hiện thị chất lượng của màn hình"),
        ("Disable clipboard", "Tắt clipboard"),
        ("Lock after session end", "Khóa sau khi kết thúc session"),
        ("Insert", "Cài"),
        ("Insert Lock", "Cài khóa"),
        ("Refresh", "Làm mới"),
        ("ID does not exist", "ID không tồn tại"),
        ("Failed to connect to rendezvous server", "Không thể kết nối đến máy chủ rendezvous"),
        ("Please try later", "Thử lại sau"),
        ("Remote desktop is offline", "Máy tính từ xa hiện đang offline"),
        ("Key mismatch", "Chìa không khớp"),
        ("Timeout", "Quá thời gian"),
        ("Failed to connect to relay server", "Không thể kết nối tới máy chủ relay"),
        ("Failed to connect via rendezvous server", "Không thể kết nối qua máy chủ rendezvous"),
        ("Failed to connect via relay server", "Không thể kết nối qua máy chủ relay"),
        ("Failed to make direct connection to remote desktop", "Không thể kết nối thẳng tới máy tính từ xa"),
        ("Set Password", "Cài đặt mật khẩu"),
        ("OS Password", "Mật khẩu hệ điều hành"),
        ("install_tip", "Do UAC, RustDesk sẽ không thể hoạt động đúng cách là bên từ xa trong vài trường hợp. Để tránh UAC, hãy nhấn cái nút dưới đây để cài RustDesk vào hệ thống."),
        ("Click to upgrade", "Nhấn để nâng cấp"),
        ("Click to download", "Nhấn để tải xuống"),
        ("Click to update", "Nhấn để cập nhật"),
        ("Configure", "Cài đặt"),
        ("config_acc", "Để có thể điều khiển máy tính từ xa, bạn cần phải cung cấp quyền \"Trợ năng\" cho RustDesk"),
        ("config_screen", "Để có thể truy cập máy tính từ xa, bạn cần phải cung cấp quyền \"Ghi Màn Hình\" cho RustDesk."),
        ("Installing ...", "Đang cài ..."),
        ("Install", "Cài"),
        ("Installation", "Cài"),
        ("Installation Path", "Địa điểm cài"),
        ("Create start menu shortcuts", "Tạo shortcut tại start menu"),
        ("Create desktop icon", "Tạo biểu tượng trên desktop"),
        ("agreement_tip", "Bằng cách bắt đầu cài đặt, bạn chấp nhận thỏa thuận cấp phép."),
        ("Accept and Install", "Chấp nhận và cài"),
        ("End-user license agreement", "Thỏa thuận cấp phép dành cho người dùng"),
        ("Generating ...", "Đang tạo ..."),
        ("Your installation is lower version.", "Phiên bản của bạn là phiên bản cũ"),
        ("not_close_tcp_tip", "Đừng đóng cửa sổ này khi bạn đang sử dụng tunnel"),
        ("Listening ...", "Đang nghe ..."),
        ("Remote Host", "Máy từ xa"),
        ("Remote Port", "Cổng từ xa"),
        ("Action", "Hành động"),
        ("Add", "Thêm"),
        ("Local Port", "Cổng nội bộ"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Để kết nối nhanh hơn, hãy tự tạo máy chủ riêng"),
        ("Too short, at least 6 characters.", "Quá ngắn, độ dài phải ít nhất là 6."),
        ("The confirmation is not identical.", "Xác minh không khớp"),
        ("Permissions", "Quyền"),
        ("Accept", "Chấp nhận"),
        ("Dismiss", "Bỏ qua"),
        ("Disconnect", "Ngắt kết nối"),
        ("Allow using keyboard and mouse", "Cho phép sử dụng bàn phím và chuột"),
        ("Allow using clipboard", "Cho phép sử dụng clipboard"),
        ("Allow hearing sound", "Cho phép nghe âm thanh"),
        ("Allow file copy and paste", "Cho phép sao chép và dán tệp tin"),
        ("Connected", "Đã kết nối"),
        ("Direct and encrypted connection", "Kết nối trực tiếp và đuợc mã hóa"),
        ("Relayed and encrypted connection", "Kết nối relay và đuợc mã hóa"),
        ("Direct and unencrypted connection", "Kết nối trực tiếp và không đuợc mã hóa"),
        ("Relayed and unencrypted connection", "Kết nối relay và không đuợc mã hóa"),
        ("Enter Remote ID", "Nhập ID từ xa"),
        ("Enter your password", "Nhập mật khẩu"),
        ("Logging in...", "Đang đăng nhập"),
        ("Enable RDP session sharing", "Cho phép chia sẻ session RDP"),
        ("Auto Login", "Tự động đăng nhập"),
        ("Enable Direct IP Access", "Cho phép truy cập trực tiếp qua IP"),
        ("Rename", "Đổi tên"),
        ("Space", "Space"),
        ("Create Desktop Shortcut", "Tạo shortcut trên desktop"),
        ("Change Path", "Đổi địa điểm"),
        ("Create Folder", "Tạo thư mục"),
        ("Please enter the folder name", "Hãy nhập tên thư mục"),
        ("Fix it", "Sửa nó"),
        ("Warning", "Cảnh báo"),
        ("Login screen using Wayland is not supported", "Màn hình đăng nhập sử dụng Wayland không đựoc hỗ trợ"),
        ("Reboot required", "Yêu cầu khởi động lại"),
        ("Unsupported display server ", "Máy chủ hiển thị không đuợc hỗ trọ"),
        ("x11 expected", "Cần x11"),
        ("Port", ""),
        ("Settings", "Cài đặt"),
        ("Username", "Tên người dùng"),
        ("Invalid port", "Cổng không hợp lệ"),
        ("Closed manually by the peer", "Đóng thủ công bởi peer"),
        ("Enable remote configuration modification", "Cho phép thay đổi cấu hình bên từ xa"),
        ("Run without install", "Chạy mà không cần cài"),
        ("Always connected via relay", "Luôn đuợc kết nối qua relay"),
        ("Always connect via relay", "Luôn kết nối qua relay"),
        ("whitelist_tip", "Chỉ có những IP đựoc cho phép mới có thể truy cập"),
        ("Login", "Đăng nhập"),
        ("Logout", "Đăng xuất"),
        ("Tags", "Tags"),
        ("Search ID", "Tìm ID"),
        ("Current Wayland display server is not supported", "Máy chủ hình ảnh Wayland hiện không đuợc hỗ trợ"),
        ("whitelist_sep", "Đuợc cách nhau bởi dấu phẩy, dấu chấm phẩy, dấu cách hay dòng mới"),
        ("Add ID", "Thêm ID"),
        ("Add Tag", "Thêm Tag"),
        ("Unselect all tags", "Hủy chọn tất cả các tag"),
        ("Network error", "Lỗi mạng"),
        ("Username missed", "Mất tên người dùng"),
        ("Password missed", "Mất mật khẩu"),
        ("Wrong credentials", "Chứng danh bị sai"),
        ("Edit Tag", "Chỉnh sửa Tag"),
        ("Unremember Password", "Quên mật khẩu"),
        ("Favorites", "Favorites"),
        ("Add to Favorites", "Thêm vào mục Favorites"),
        ("Remove from Favorites", "Xóa khỏi mục Favorites"),
        ("Empty", "Trống"),
        ("Invalid folder name", "Tên thư mục không hợp lệ"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Tên host"),
        ("Discovered", "Đuợc phát hiện"),
        ("install_daemon_tip", "Để chạy lúc khởi động máy, bạn cần phải cài dịch vụ hệ thống."),
        ("Remote ID", "ID từ xa"),
        ("Paste", "Dán"),
        ("Paste here?", "Dán ở đây?"),
        ("Are you sure to close the connection?", "Bạn có chắc muốn đóng kết nối không"),
        ("Download new version", "Tải về phiên bản mới"),
        ("Touch mode", "Chế độ chạm"),
        ("Mouse mode", "Chế độ dùng chuột"),
        ("One-Finger Tap", "Chạm bằng một ngón tay"),
        ("Left Mouse", "Chuột trái"),
        ("One-Long Tap", "Chạm lâu bằng một ngón tay"),
        ("Two-Finger Tap", "Chạm bằng hai ngón tay"),
        ("Right Mouse", "Chuột phải"),
        ("One-Finger Move", "Di chuyển bằng một ngón tay"),
        ("Double Tap & Move", "Chạm hai lần và di chuyển"),
        ("Mouse Drag", "Di chuyển bằng chuột"),
        ("Three-Finger vertically", "Ba ngón tay theo chiều dọc"),
        ("Mouse Wheel", "Bánh xe lăn trê con chuột"),
        ("Two-Finger Move", "Di chuyển bằng hai ngón tay"),
        ("Canvas Move", "Di chuyển canvas"),
        ("Pinch to Zoom", "Véo để phóng to/nhỏ"),
        ("Canvas Zoom", "Phóng to/nhỏ canvas"),
        ("Reset canvas", "Cài đặt lại canvas"),
        ("No permission of file transfer", "Không có quyền truyền tệp tin"),
        ("Note", "Ghi nhớ"),
        ("Connection", "Kết nối"),
        ("Share Screen", "Chia sẻ màn hình"),
        ("CLOSE", "ĐÓNG"),
        ("OPEN", "MỞ"),
        ("Chat", "Chat"),
        ("Total", "Tổng"),
        ("items", "items"),
        ("Selected", "Đã đuợc chọn"),
        ("Screen Capture", "Ghi màn hình"),
        ("Input Control", "Điều khiển đầu vào"),
        ("Audio Capture", "Ghi âm thanh"),
        ("File Connection", "Kết nối tệp tin"),
        ("Screen Connection", "Kết nối màn hình"),
        ("Do you accept?", "Bạn có chấp nhận không?"),
        ("Open System Setting", "Mở cài đặt hệ thống"),
        ("How to get Android input permission?", "Cách để có quyền nhập trên Android?"),
        ("android_input_permission_tip1", "Để thiết bị từ xa điều khiển thiết bị Android của bạn bằng chuột hoặc chạm, bạn cần cho phép RustDesk sử dụng dịch vụ \"Trợ năng\"."),
        ("android_input_permission_tip2", "Vui lòng chuyển đến trang cài đặt hệ thống tiếp theo, tìm và nhập [Dịch vụ đã cài đặt], bật dịch vụ [RustDesk Input]."),
        ("android_new_connection_tip", "Yêu cầu kiểm soát mới đã được nhận, yêu cầu này muốn kiểm soát thiết bị hiện tại của bạn."),
        ("android_service_will_start_tip", "Bật \"Ghi màn hình\" sẽ tự động khởi động dịch vụ, cho phép các thiết bị khác yêu cầu kết nối với thiết bị của bạn."),
        ("android_stop_service_tip", "Đóng dịch vụ sẽ tự động đóng tất cả các kết nối đã thiết lập."),
        ("android_version_audio_tip", "Phiên bản Android hiện tại không hỗ trợ ghi âm, vui lòng nâng cấp lên Android 10 trở lên."),
        ("android_start_service_tip", "Nhấn vào [Bắt đầu dịch vụ] hoặc MỞ quyền [Ghi màn hình] để bắt đầu dịch vụ chia sẻ màn hình."),
        ("Account", ""),
        ("Overwrite", "Ghi đè"),
        ("This file exists, skip or overwrite this file?", "Tệp tin này đã tồn tại, bạn có muốn bỏ qua hay ghi đè lên tệp tin này?"),
        ("Quit", "Thoát"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Giúp đỡ"),
        ("Failed", "Thất bại"),
        ("Succeeded", "Thành công"),
        ("Someone turns on privacy mode, exit", "Ai đó đã bật chế độ riêng tư, thoát"),
        ("Unsupported", "Không hỗ trợ"),
        ("Peer denied", "Peer đã từ chối"),
        ("Please install plugins", "Hãy cài plugins"),
        ("Peer exit", "Peer đã thoát"),
        ("Failed to turn off", "Không thể tắt"),
        ("Turned off", "Đã tắt"),
        ("In privacy mode", "Vào chế độ riêng tư"),
        ("Out privacy mode", "Thoát chế độ riêng tư"),
        ("Language", "Ngôn ngữ"),
        ("Keep RustDesk background service", "Giữ dịch vụ nền RustDesk"),
        ("Ignore Battery Optimizations", "Bỏ qua các tối ưu pin"),
        ("android_open_battery_optimizations_tip", "Nếu bạn muốn tắt tính năng này, vui lòng chuyển đến trang cài đặt ứng dụng RustDesk tiếp theo, tìm và nhập [Pin], Bỏ chọn [Không hạn chế]"),
        ("Connection not allowed", "Kết nối không đuợc phép"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", "Sử dụng mật khẩu vĩnh viễn"),
        ("Use both passwords", "Sử dụng cả hai mật khẩu"),
        ("Set permanent password", "Đặt mật khẩu vĩnh viễn"),
        ("Enable Remote Restart", "Bật khởi động lại từ xa"),
        ("Allow remote restart", "Cho phép khởi động lại từ xa"),
        ("Restart Remote Device", "Khởi động lại thiết bị từ xa"),
        ("Are you sure you want to restart", "Bạn có chắc bạn muốn khởi động lại không"),
        ("Restarting Remote Device", "Đang khởi động lại thiết bị từ xa"),
        ("remote_restarting_tip", "Thiết bị từ xa đang khởi động lại, hãy đóng cửa sổ tin nhắn này và kết nối lại với mật khẩu vĩnh viễn sau một khoảng thời gian"),
        ("Copied", ""),
        ("Exit Fullscreen", "Thoát toàn màn hình"),
        ("Fullscreen", "Toàn màn hình"),
        ("Mobile Actions", "Hành động trên thiết bị di động"),
        ("Select Monitor", "Chọn màn hình"),
        ("Control Actions", "Kiểm soát hành động"),
        ("Display Settings", "Thiết lập hiển thị"),
        ("Ratio", "Tỉ lệ"),
        ("Image Quality", "Chất lượng hình ảnh"),
        ("Scroll Style", "Kiểu cuộn"),
        ("Show Menubar", "Hiển thị thanh menu"),
        ("Hide Menubar", "ẩn thanh menu"),
        ("Direct Connection", "Kết nối trực tiếp"),
        ("Relay Connection", "Kết nối chuyển tiếp"),
        ("Secure Connection", "Kết nối an toàn"),
        ("Insecure Connection", "Kết nối không an toàn"),
        ("Scale original", "Quy mô gốc"),
        ("Scale adaptive", "Quy mô thích ứng"),
        ("General", ""),
        ("Security", ""),
        ("Account", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Port", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Deny remote access", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin menubar", "Ghim thanh menu"),
        ("Unpin menubar", "Bỏ ghim thanh menu"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Custom", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland yêu cầu phiên bản Ubuntu 21.04 trở lên."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland yêu cầu phiên bản distro linux cao hơn. Vui lòng thử máy tính để bàn X11 hoặc thay đổi hệ điều hành của bạn."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vui lòng Chọn màn hình để chia sẻ (Hoạt động ở phía ngang hàng)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
    ].iter().cloned().collect();
}
