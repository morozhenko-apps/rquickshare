//! Security-sensitive direct GTK settings warnings.
//! Presentation only; neither controller matching nor acknowledgements change.

pub(super) const KEYS: [&str; 12] = [
    "Each incoming transfer still requires your confirmation. Folder changes affect new requests.",
    "Leave empty or use 0 for automatic selection. A changed port requires restarting sharing when idle.",
    "Sign-in state changed or is unavailable. Reload it before confirming.",
    "Device name is too long or contains unsupported characters or surrounding spaces. Leave empty for the automatic name.",
    "Sharing settings are unavailable. Retry the connection from the card.",
    "Sign-in change was not confirmed. Any acknowledged sharing settings remain saved.",
    "Choose a visibility setting before saving.",
    "Port must be empty or an integer from 0 to 65535.",
    "Cannot create a save identity. Try again.",
    "Cannot retain the selected folder. Choose it again.",
    "Settings acknowledgement was incomplete. Reload before making another edit.",
    "Cannot acquire the selected folder. Check access and choose again.",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 12] = match locale {
        "ru" => &[
            "Каждая входящая передача всё равно требует вашего подтверждения. Изменение папки затрагивает новые запросы.",
            "Оставьте поле пустым или укажите 0 для автоматического выбора. После смены порта перезапустите обмен, когда не будет передач.",
            "Состояние автозапуска изменилось или недоступно. Обновите его перед подтверждением.",
            "Имя устройства слишком длинное, содержит недопустимые символы или пробелы по краям. Оставьте поле пустым для автоматического имени.",
            "Настройки обмена недоступны. Повторите подключение из карточки.",
            "Изменение автозапуска не подтверждено. Ранее подтверждённые настройки обмена остаются сохранёнными.",
            "Выберите видимость перед сохранением.",
            "Порт должен быть пустым полем или целым числом от 0 до 65535.",
            "Не удалось создать идентификатор сохранения. Повторите попытку.",
            "Не удалось сохранить доступ к выбранной папке. Выберите её повторно.",
            "Подтверждение настроек неполное. Обновите данные перед следующей правкой.",
            "Не удалось получить доступ к выбранной папке. Проверьте разрешения и выберите её снова.",
        ],
        "es" => &[
            "Cada transferencia entrante sigue requiriendo tu confirmación. Los cambios de carpeta afectan a las solicitudes nuevas.",
            "Deja el campo vacío o usa 0 para seleccionar automáticamente. Si cambias el puerto, reinicia el servicio cuando esté inactivo.",
            "El estado de inicio de sesión cambió o no está disponible. Recarga antes de confirmar.",
            "El nombre del dispositivo es demasiado largo, contiene caracteres no admitidos o espacios en los extremos. Déjalo vacío para usar el nombre automático.",
            "Los ajustes para compartir no están disponibles. Vuelve a conectar desde la tarjeta.",
            "El cambio de inicio de sesión no está confirmado. Los ajustes de uso compartido ya confirmados siguen guardados.",
            "Elige una visibilidad antes de guardar.",
            "El puerto debe estar vacío o ser un entero de 0 a 65535.",
            "No se pudo crear una identidad de guardado. Inténtalo de nuevo.",
            "No se pudo conservar el acceso a la carpeta elegida. Selecciónala de nuevo.",
            "La confirmación de los ajustes está incompleta. Recarga antes de hacer otra modificación.",
            "No se pudo acceder a la carpeta seleccionada. Comprueba los permisos y vuelve a elegirla.",
        ],
        "pt" => &[
            "Cada transferência recebida continua a exigir a sua confirmação. As alterações à pasta afetam os novos pedidos.",
            "Deixe vazio ou use 0 para seleção automática. Após alterar a porta, reinicie a partilha quando estiver inativa.",
            "O estado de arranque mudou ou está indisponível. Recarregue antes de confirmar.",
            "O nome do dispositivo é demasiado longo, contém caracteres não suportados ou espaços nas extremidades. Deixe vazio para usar o nome automático.",
            "As definições de partilha estão indisponíveis. Repita a ligação a partir do cartão.",
            "A alteração do arranque não foi confirmada. As definições de partilha já confirmadas continuam guardadas.",
            "Escolha uma visibilidade antes de guardar.",
            "A porta deve estar vazia ou conter um número inteiro de 0 a 65535.",
            "Não foi possível criar a identidade da gravação. Tente novamente.",
            "Não foi possível manter o acesso à pasta selecionada. Escolha-a novamente.",
            "A confirmação das definições está incompleta. Recarregue antes de fazer outra alteração.",
            "Não foi possível aceder à pasta selecionada. Verifique as permissões e escolha-a novamente.",
        ],
        "de" => &[
            "Jede eingehende Übertragung erfordert weiterhin Ihre Bestätigung. Ordneränderungen gelten für neue Anfragen.",
            "Leer lassen oder 0 für automatische Auswahl eingeben. Nach einer Portänderung die Freigabe im Leerlauf neu starten.",
            "Der Autostartstatus hat sich geändert oder ist nicht verfügbar. Laden Sie ihn vor der Bestätigung neu.",
            "Der Gerätename ist zu lang, enthält unzulässige Zeichen oder Leerzeichen an den Rändern. Für den automatischen Namen leer lassen.",
            "Freigabeeinstellungen sind nicht verfügbar. Verbinden Sie sich über die Karte erneut.",
            "Die Autostartänderung ist nicht bestätigt. Bereits bestätigte Freigabeeinstellungen bleiben gespeichert.",
            "Wählen Sie vor dem Speichern eine Sichtbarkeit.",
            "Der Port muss leer sein oder eine ganze Zahl von 0 bis 65535 enthalten.",
            "Eine Speicherkennung konnte nicht erstellt werden. Versuchen Sie es erneut.",
            "Der Zugriff auf den gewählten Ordner konnte nicht erhalten bleiben. Wählen Sie ihn erneut.",
            "Die Bestätigung der Einstellungen ist unvollständig. Laden Sie vor weiteren Änderungen neu.",
            "Auf den gewählten Ordner konnte nicht zugegriffen werden. Prüfen Sie die Berechtigungen und wählen Sie ihn erneut.",
        ],
        "fr" => &[
            "Chaque transfert entrant nécessite toujours votre confirmation. Les changements de dossier concernent les nouvelles demandes.",
            "Laissez vide ou saisissez 0 pour la sélection automatique. Après un changement de port, redémarrez le partage au repos.",
            "L’état du démarrage a changé ou est indisponible. Rechargez-le avant de confirmer.",
            "Le nom de l’appareil est trop long, contient des caractères non autorisés ou des espaces au début ou à la fin. Laissez vide pour un nom automatique.",
            "Les paramètres de partage sont indisponibles. Reconnectez-vous depuis la carte.",
            "Le changement du démarrage n’est pas confirmé. Les paramètres de partage déjà confirmés restent enregistrés.",
            "Choisissez une visibilité avant d’enregistrer.",
            "Le port doit être vide ou être un entier de 0 à 65535.",
            "Impossible de créer un identifiant d’enregistrement. Réessayez.",
            "Impossible de conserver l’accès au dossier choisi. Sélectionnez-le à nouveau.",
            "La confirmation des paramètres est incomplète. Rechargez avant toute autre modification.",
            "Impossible d’accéder au dossier choisi. Vérifiez les autorisations et sélectionnez-le à nouveau.",
        ],
        "it" => &[
            "Ogni trasferimento in arrivo richiede ancora una conferma. Le modifiche alla cartella valgono per le nuove richieste.",
            "Lascia vuoto o usa 0 per la selezione automatica. Dopo aver cambiato porta, riavvia la condivisione quando inattiva.",
            "Lo stato di avvio è cambiato o non è disponibile. Ricaricalo prima di confermare.",
            "Il nome del dispositivo è troppo lungo o contiene caratteri non ammessi o spazi alle estremità. Lascialo vuoto per il nome automatico.",
            "Le impostazioni di condivisione non sono disponibili. Riconnettiti dalla scheda.",
            "La modifica dell’avvio non è confermata. Le impostazioni di condivisione già confermate restano salvate.",
            "Scegli la visibilità prima di salvare.",
            "La porta deve essere vuota o un numero intero da 0 a 65535.",
            "Impossibile creare un’identità di salvataggio. Riprova.",
            "Impossibile mantenere l’accesso alla cartella scelta. Selezionala di nuovo.",
            "La conferma delle impostazioni è incompleta. Ricarica prima di altre modifiche.",
            "Impossibile accedere alla cartella scelta. Controlla i permessi e selezionala di nuovo.",
        ],
        "pl" => &[
            "Każdy przychodzący transfer nadal wymaga potwierdzenia. Zmiana folderu dotyczy nowych żądań.",
            "Pozostaw puste lub wpisz 0, aby wybrać automatycznie. Po zmianie portu uruchom udostępnianie ponownie w stanie bezczynności.",
            "Stan autostartu zmienił się lub jest niedostępny. Wczytaj go ponownie przed potwierdzeniem.",
            "Nazwa urządzenia jest zbyt długa, zawiera niedozwolone znaki lub spacje na brzegach. Pozostaw puste, aby użyć nazwy automatycznej.",
            "Ustawienia udostępniania są niedostępne. Połącz się ponownie z karty.",
            "Zmiana autostartu nie została potwierdzona. Wcześniej potwierdzone ustawienia udostępniania pozostały zapisane.",
            "Wybierz widoczność przed zapisaniem.",
            "Port musi być pusty lub zawierać liczbę całkowitą od 0 do 65535.",
            "Nie udało się utworzyć identyfikatora zapisu. Spróbuj ponownie.",
            "Nie udało się zachować dostępu do wybranego folderu. Wybierz go ponownie.",
            "Potwierdzenie ustawień jest niekompletne. Wczytaj ponownie przed kolejną zmianą.",
            "Nie można uzyskać dostępu do wybranego folderu. Sprawdź uprawnienia i wybierz go ponownie.",
        ],
        "hi" => &[
            "हर आने वाले ट्रांसफ़र के लिए अब भी आपकी पुष्टि आवश्यक है। फ़ोल्डर बदलने से नए अनुरोध प्रभावित होते हैं।",
            "स्वतः चयन के लिए खाली छोड़ें या 0 दर्ज करें। पोर्ट बदलने के बाद खाली समय में शेयरिंग फिर शुरू करें।",
            "साइन-इन पर शुरू होने की स्थिति बदली है या उपलब्ध नहीं है। पुष्टि से पहले फिर लोड करें।",
            "डिवाइस का नाम बहुत लंबा है या उसमें अमान्य वर्ण या किनारों पर स्पेस हैं। स्वतः नाम के लिए खाली छोड़ें।",
            "शेयरिंग सेटिंग्स उपलब्ध नहीं हैं। कार्ड से फिर कनेक्ट करें।",
            "साइन-इन पर शुरू होने का बदलाव पुष्टि नहीं हुआ है। पहले से पुष्टि की गई शेयरिंग सेटिंग्स सहेजी हुई हैं।",
            "सेव करने से पहले दृश्यता चुनें।",
            "पोर्ट खाली होना चाहिए या 0 से 65535 तक पूर्णांक होना चाहिए।",
            "सेव पहचान नहीं बनाई जा सकी। फिर कोशिश करें।",
            "चुने गए फ़ोल्डर की पहुँच बरकरार नहीं रह सकी। इसे फिर चुनें।",
            "सेटिंग्स की पुष्टि अधूरी है। दूसरा बदलाव करने से पहले फिर लोड करें।",
            "चुने गए फ़ोल्डर तक पहुँच नहीं मिली। अनुमतियाँ जाँचें और फिर चुनें।",
        ],
        "zh-Hant" => &[
            "每次接收傳輸仍需您確認。資料夾變更只影響新的請求。",
            "留空或輸入 0 以自動選擇。變更連接埠後，請在閒置時重新啟動分享。",
            "登入啟動狀態已變更或無法使用。請在確認前重新載入。",
            "裝置名稱過長，包含不支援的字元或首尾空格。留空即可使用自動名稱。",
            "分享設定無法使用。請從卡片重新連線。",
            "登入啟動變更尚未確認。先前已確認的分享設定仍會保留。",
            "儲存前請選擇可見性。",
            "連接埠必須留空或填入 0 到 65535 的整數。",
            "無法建立儲存識別碼。請重試。",
            "無法保留所選資料夾的存取權。請重新選擇。",
            "設定確認尚未完成。再次修改前請重新載入。",
            "無法存取所選資料夾。請檢查權限並重新選擇。",
        ],
        "ja" => &[
            "受信する転送には引き続き確認が必要です。フォルダーの変更は新しい要求に適用されます。",
            "自動選択するには空欄にするか 0 を指定します。ポート変更後は待機中に共有を再起動してください。",
            "自動起動の状態が変更されたか利用できません。確認前に再読み込みしてください。",
            "デバイス名が長すぎるか、使えない文字や先頭・末尾の空白を含みます。自動名を使用するには空欄にしてください。",
            "共有設定を利用できません。カードから再接続してください。",
            "自動起動の変更は確認されていません。確認済みの共有設定は保存されたままです。",
            "保存前に公開範囲を選択してください。",
            "ポートは空欄にするか 0 から 65535 の整数にしてください。",
            "保存用の識別子を作成できませんでした。再試行してください。",
            "選択したフォルダーへのアクセスを保持できませんでした。再選択してください。",
            "設定の確認が完了していません。別の変更をする前に再読み込みしてください。",
            "選択したフォルダーにアクセスできません。権限を確認して再選択してください。",
        ],
        "nl" => &[
            "Elke inkomende overdracht vereist nog steeds uw bevestiging. Mapwijzigingen gelden voor nieuwe verzoeken.",
            "Laat leeg of gebruik 0 voor automatische selectie. Start delen opnieuw wanneer er geen activiteit is na een poortwijziging.",
            "De opstartstatus is gewijzigd of niet beschikbaar. Laad opnieuw voordat u bevestigt.",
            "De apparaatnaam is te lang, bevat ongeldige tekens of spaties aan de randen. Laat leeg voor een automatische naam.",
            "Instellingen voor delen zijn niet beschikbaar. Verbind opnieuw via de kaart.",
            "De opstartwijziging is niet bevestigd. Eerder bevestigde instellingen voor delen blijven opgeslagen.",
            "Kies zichtbaarheid voordat u opslaat.",
            "De poort moet leeg zijn of een geheel getal van 0 tot 65535 bevatten.",
            "Kon geen opslagidentiteit maken. Probeer opnieuw.",
            "Kon toegang tot de geselecteerde map niet behouden. Kies deze opnieuw.",
            "De bevestiging van de instellingen is onvolledig. Laad opnieuw voordat u iets wijzigt.",
            "Geen toegang tot de geselecteerde map. Controleer de rechten en kies opnieuw.",
        ],
        "sv" => &[
            "Varje inkommande överföring kräver fortfarande ditt godkännande. Mappändringar gäller nya begäranden.",
            "Lämna tomt eller ange 0 för automatiskt val. Starta om delning vid inaktivitet efter portbyte.",
            "Autostartstatus har ändrats eller är inte tillgänglig. Läs in den igen före bekräftelse.",
            "Enhetsnamnet är för långt, har otillåtna tecken eller blanksteg i början/slutet. Lämna tomt för ett automatiskt namn.",
            "Delningsinställningarna är inte tillgängliga. Anslut igen via kortet.",
            "Autostartändringen är inte bekräftad. Tidigare bekräftade delningsinställningar förblir sparade.",
            "Välj synlighet före sparande.",
            "Porten måste vara tom eller ett heltal mellan 0 och 65535.",
            "Kunde inte skapa en sparidentitet. Försök igen.",
            "Kunde inte behålla åtkomsten till vald mapp. Välj den igen.",
            "Bekräftelsen av inställningarna är ofullständig. Läs in igen innan fler ändringar.",
            "Kan inte komma åt den valda mappen. Kontrollera behörigheterna och välj igen.",
        ],
        "tr" => &[
            "Gelen her aktarım yine de onayınızı gerektirir. Klasör değişiklikleri yeni isteklere uygulanır.",
            "Otomatik seçim için boş bırakın veya 0 girin. Bağlantı noktası değişirse paylaşımı boşta iken yeniden başlatın.",
            "Girişte başlatma durumu değişti veya kullanılamıyor. Onaylamadan önce yeniden yükleyin.",
            "Cihaz adı çok uzun, desteklenmeyen karakterler veya başta/sonda boşluklar içeriyor. Otomatik ad için boş bırakın.",
            "Paylaşım ayarları kullanılamıyor. Karttan yeniden bağlanın.",
            "Girişte başlatma değişikliği doğrulanmadı. Önceden onaylanan paylaşım ayarları kayıtlı kalır.",
            "Kaydetmeden önce görünürlüğü seçin.",
            "Bağlantı noktası boş veya 0 ile 65535 arasında bir tam sayı olmalıdır.",
            "Kaydetme kimliği oluşturulamadı. Yeniden deneyin.",
            "Seçili klasöre erişim korunamadı. Yeniden seçin.",
            "Ayar onayı tamamlanmadı. Başka bir değişiklikten önce yeniden yükleyin.",
            "Seçili klasöre erişilemiyor. İzinleri kontrol edip yeniden seçin.",
        ],
        "ko" => &[
            "들어오는 각 전송에는 여전히 확인이 필요합니다. 폴더 변경은 새 요청에 적용됩니다.",
            "자동 선택하려면 비워 두거나 0을 입력하세요. 포트를 변경한 뒤에는 전송이 없을 때 공유를 다시 시작하세요.",
            "로그인 시 시작 상태가 변경되었거나 사용할 수 없습니다. 확인하기 전에 다시 불러오세요.",
            "기기 이름이 너무 길거나 지원하지 않는 문자 또는 앞뒤 공백이 있습니다. 자동 이름을 사용하려면 비워 두세요.",
            "공유 설정을 사용할 수 없습니다. 카드에서 다시 연결하세요.",
            "로그인 시작 변경이 확인되지 않았습니다. 이미 확인된 공유 설정은 저장된 상태로 유지됩니다.",
            "저장하기 전에 공개 상태를 선택하세요.",
            "포트는 비워 두거나 0부터 65535까지의 정수여야 합니다.",
            "저장 식별자를 만들 수 없습니다. 다시 시도하세요.",
            "선택한 폴더에 대한 접근을 유지하지 못했습니다. 다시 선택하세요.",
            "설정 확인이 완료되지 않았습니다. 다시 변경하기 전에 불러오세요.",
            "선택한 폴더에 접근할 수 없습니다. 권한을 확인하고 다시 선택하세요.",
        ],
        "bn" => &[
            "প্রতিটি আগত স্থানান্তরের জন্য এখনও আপনার সম্মতি প্রয়োজন। ফোল্ডার বদলালে নতুন অনুরোধ প্রভাবিত হয়।",
            "স্বয়ংক্রিয় নির্বাচনের জন্য খালি রাখুন বা 0 দিন। পোর্ট বদলালে অব্যবহৃত অবস্থায় শেয়ারিং আবার চালু করুন।",
            "লগইন স্টার্টের অবস্থা বদলেছে বা পাওয়া যাচ্ছে না। নিশ্চিত করার আগে আবার লোড করুন।",
            "ডিভাইসের নাম খুব বড় বা অসমর্থিত অক্ষর কিংবা শুরু/শেষে ফাঁকা স্থান আছে। স্বয়ংক্রিয় নামের জন্য খালি রাখুন।",
            "শেয়ারিং সেটিংস পাওয়া যাচ্ছে না। কার্ড থেকে আবার সংযুক্ত হন।",
            "লগইন স্টার্ট পরিবর্তন নিশ্চিত হয়নি। আগে নিশ্চিত করা শেয়ারিং সেটিংস সংরক্ষিত আছে।",
            "সংরক্ষণের আগে দৃশ্যমানতা বেছে নিন।",
            "পোর্ট খালি বা 0 থেকে 65535 পর্যন্ত পূর্ণসংখ্যা হতে হবে।",
            "সংরক্ষণের পরিচয় তৈরি করা যায়নি। আবার চেষ্টা করুন।",
            "নির্বাচিত ফোল্ডারে প্রবেশাধিকার ধরে রাখা যায়নি। আবার বেছে নিন।",
            "সেটিংসের নিশ্চিতকরণ অসম্পূর্ণ। অন্য পরিবর্তনের আগে আবার লোড করুন।",
            "নির্বাচিত ফোল্ডারে প্রবেশ করা যায়নি। অনুমতি যাচাই করে আবার বেছে নিন।",
        ],
        "th" => &[
            "การถ่ายโอนขาเข้าทุกครั้งยังต้องได้รับการยืนยันจากคุณ การเปลี่ยนโฟลเดอร์มีผลกับคำขอใหม่",
            "ปล่อยว่างหรือป้อน 0 เพื่อเลือกอัตโนมัติ หลังเปลี่ยนพอร์ตให้เริ่มการแชร์ใหม่เมื่อไม่มีการถ่ายโอน",
            "สถานะเริ่มเมื่อเข้าสู่ระบบเปลี่ยนหรือใช้ไม่ได้ โหลดใหม่ก่อนยืนยัน",
            "ชื่ออุปกรณ์ยาวเกินไป มีอักขระไม่รองรับหรือช่องว่างหน้าและหลัง ปล่อยว่างเพื่อใช้ชื่ออัตโนมัติ",
            "การตั้งค่าการแชร์ใช้ไม่ได้ เชื่อมต่ออีกครั้งจากการ์ด",
            "การเปลี่ยนการเริ่มเมื่อเข้าสู่ระบบยังไม่ยืนยัน การตั้งค่าการแชร์ที่ยืนยันแล้วจะยังบันทึกอยู่",
            "เลือกการมองเห็นก่อนบันทึก",
            "พอร์ตต้องว่างหรือเป็นจำนวนเต็มตั้งแต่ 0 ถึง 65535",
            "ไม่สามารถสร้างข้อมูลระบุการบันทึกได้ ลองอีกครั้ง",
            "ไม่สามารถรักษาสิทธิ์เข้าถึงโฟลเดอร์ที่เลือกไว้ได้ เลือกใหม่",
            "การยืนยันการตั้งค่าไม่สมบูรณ์ โหลดใหม่ก่อนแก้ไขอีกครั้ง",
            "เข้าโฟลเดอร์ที่เลือกไม่ได้ ตรวจสอบสิทธิ์แล้วเลือกใหม่",
        ],
        "id" => &[
            "Setiap transfer masuk tetap membutuhkan persetujuan Anda. Perubahan folder berlaku untuk permintaan baru.",
            "Kosongkan atau masukkan 0 untuk pemilihan otomatis. Setelah mengganti port, mulai ulang berbagi saat tidak sibuk.",
            "Status mulai saat masuk berubah atau tidak tersedia. Muat ulang sebelum mengonfirmasi.",
            "Nama perangkat terlalu panjang, berisi karakter yang tidak didukung atau spasi di awal/akhir. Biarkan kosong untuk nama otomatis.",
            "Pengaturan berbagi tidak tersedia. Sambungkan kembali dari kartu.",
            "Perubahan mulai saat masuk belum dikonfirmasi. Pengaturan berbagi yang sebelumnya dikonfirmasi tetap tersimpan.",
            "Pilih visibilitas sebelum menyimpan.",
            "Port harus kosong atau berupa bilangan bulat dari 0 sampai 65535.",
            "Tidak dapat membuat identitas penyimpanan. Coba lagi.",
            "Tidak dapat mempertahankan akses ke folder terpilih. Pilih lagi.",
            "Konfirmasi pengaturan belum lengkap. Muat ulang sebelum membuat perubahan lain.",
            "Tidak dapat mengakses folder terpilih. Periksa izin dan pilih lagi.",
        ],
        "fil" => &[
            "Kailangan pa rin ang iyong kumpirmasyon sa bawat papasok na paglilipat. Para sa mga bagong kahilingan ang pagbabago ng folder.",
            "Iwang blangko o gumamit ng 0 para sa awtomatikong pagpili. I-restart ang sharing kapag walang gawain matapos palitan ang port.",
            "Nagbago o hindi available ang estado ng pagsisimula sa pag-sign in. I-reload bago kumpirmahin.",
            "Masyadong mahaba ang pangalan ng device, may hindi sinusuportahang character o espasyo sa magkabilang dulo. Iwang blangko para sa awtomatikong pangalan.",
            "Hindi available ang mga setting sa pagbabahagi. Kumonekta muli mula sa card.",
            "Hindi nakumpirma ang pagbabago sa startup. Naka-save pa rin ang dati nang nakumpirmang mga setting sa pagbabahagi.",
            "Piliin ang visibility bago mag-save.",
            "Dapat blangko ang port o isang buong bilang mula 0 hanggang 65535.",
            "Hindi makagawa ng pagkakakilanlan sa pag-save. Subukang muli.",
            "Hindi mapanatili ang access sa napiling folder. Piliin ito muli.",
            "Hindi kumpleto ang kumpirmasyon ng mga setting. I-reload bago gumawa ng ibang pagbabago.",
            "Hindi ma-access ang napiling folder. Tingnan ang mga pahintulot at pumili muli.",
        ],
        "ar" => &[
            "تظل كل عملية نقل واردة بحاجة إلى موافقتك. تؤثر تغييرات المجلد في الطلبات الجديدة.",
            "اترك الحقل فارغًا أو أدخل 0 للاختيار التلقائي. بعد تغيير المنفذ، أعد تشغيل المشاركة حين لا توجد عمليات نقل.",
            "تغيّرت حالة بدء التشغيل عند الدخول أو أصبحت غير متاحة. أعد تحميلها قبل التأكيد.",
            "اسم الجهاز طويل جدًا أو يحتوي على رموز غير مدعومة أو مسافات في بدايته أو نهايته. اتركه فارغًا للاسم التلقائي.",
            "إعدادات المشاركة غير متاحة. أعد الاتصال من البطاقة.",
            "لم يتم تأكيد تغيير بدء التشغيل عند الدخول. تظل إعدادات المشاركة المؤكدة سابقًا محفوظة.",
            "اختر إعداد الظهور قبل الحفظ.",
            "يجب ترك المنفذ فارغًا أو إدخال عدد صحيح من 0 إلى 65535.",
            "تعذّر إنشاء معرّف للحفظ. حاول مجددًا.",
            "تعذّر الاحتفاظ بإمكانية الوصول إلى المجلد المحدد. اختره مجددًا.",
            "لم يكتمل تأكيد الإعدادات. أعد التحميل قبل أي تعديل آخر.",
            "تعذّر الوصول إلى المجلد المحدد. تحقق من الأذونات واختره مجددًا.",
        ],
        _ => &KEYS,
    };
    Some(row[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn direct_warning_catalog_complete_and_falls_back_safely() {
        for (index, source) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(source), "duplicate warning");
            assert_eq!(translated("en", source), Some(*source));
            for locale in LOCALES {
                let text = translated(locale, source).expect("warning key");
                assert!(!text.trim().is_empty(), "{locale}: {source}");
                assert!(!text.contains('{'), "{locale}: {source}");
            }
        }
        assert_eq!(translated("pt-BR", KEYS[7]), Some(KEYS[7]));
        assert_eq!(translated("zh-Hans", KEYS[7]), Some(KEYS[7]));
        assert_eq!(translated("invalid", "Peer file name"), None);
    }

    #[test]
    fn all_locales_distinguish_consent_from_errors_and_retain_port_bounds() {
        for locale in LOCALES {
            let get = |key| translated(locale, key).expect("direct settings warning");
            assert!(get(KEYS[1]).contains("0"), "{locale}: automatic port");
            assert!(get(KEYS[7]).contains("0"), "{locale}: minimum port");
            assert!(get(KEYS[7]).contains("65535"), "{locale}: maximum port");
            for (a, b) in [
                (0, 1),
                (2, 5),
                (4, 10),
                (5, 10),
                (6, 7),
                (8, 9),
                (9, 11),
                (10, 11),
            ] {
                assert_ne!(get(KEYS[a]), get(KEYS[b]), "{locale}: warning collision");
            }
        }
        assert_eq!(translated("ru", KEYS[6]), Some("Выберите видимость перед сохранением."));
        assert_eq!(translated("pt", KEYS[7]), Some("A porta deve estar vazia ou conter um número inteiro de 0 a 65535."));
    }
}
