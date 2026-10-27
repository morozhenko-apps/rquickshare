//! Immutable presentation-only client error and uncertain-completion messages.
//! Error classification, persistence, delivery, consent and retry remain in the controller.

pub(super) const KEYS: [&str; 12] = [
    "Settings save outcome is uncertain. Open Settings and retry the pending save.",
    "Settings changed. Open Settings and reload before saving.",
    "Settings need attention. Open Settings to review the result.",
    "Service connection lost. Reconnect starts a fresh client; choose unsent files again.",
    "Service is incompatible or returned invalid state. Update it, then reconnect.",
    "Operation needs recovery. Do not repeat the send.",
    "Cannot read the selected files. Check access and select them again.",
    "A recipient is no longer available. Review the selected devices; nothing was queued.",
    "Sharing resources are busy. Nothing was queued. Wait for a queued transfer to finish.",
    "The operation was not accepted. Review the current state and try again.",
    "Visibility was saved but could not be fully applied. Review discovery status and Settings.",
    "Operation did not complete. Review its state before retrying.",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 12] = match locale {
        "ru" => &[
            "Результат сохранения настроек неизвестен. Откройте настройки и повторите ожидающее сохранение.",
            "Настройки изменились. Откройте настройки и обновите их перед сохранением.",
            "Настройки требуют внимания. Откройте настройки и проверьте результат.",
            "Соединение со службой потеряно. Повторное подключение создаст новый клиент; снова выберите неотправленные файлы.",
            "Служба несовместима или вернула недопустимое состояние. Обновите её и подключитесь снова.",
            "Операцию необходимо восстановить. Не повторяйте отправку.",
            "Не удалось прочитать выбранные файлы. Проверьте права доступа и выберите их снова.",
            "Получатель больше недоступен. Проверьте выбранные устройства; ничего не поставлено в очередь.",
            "Ресурсы передачи заняты. Ничего не поставлено в очередь. Дождитесь завершения передачи из очереди.",
            "Операция не принята. Проверьте текущее состояние и повторите попытку.",
            "Видимость сохранена, но применена не полностью. Проверьте состояние обнаружения и настройки.",
            "Операция не завершилась. Проверьте её состояние, прежде чем повторять попытку.",
        ],
        "es" => &[
            "El resultado de guardar los ajustes es incierto. Abre Ajustes y reintenta el guardado pendiente.",
            "Los ajustes cambiaron. Abre Ajustes y vuelve a cargarlos antes de guardar.",
            "Los ajustes necesitan revisión. Abre Ajustes y comprueba el resultado.",
            "Se perdió la conexión con el servicio. Reconectar crea un cliente nuevo; vuelve a elegir los archivos no enviados.",
            "El servicio es incompatible o devolvió un estado no válido. Actualízalo y vuelve a conectar.",
            "La operación necesita recuperación. No repitas el envío.",
            "No se pueden leer los archivos elegidos. Comprueba el acceso y selecciónalos de nuevo.",
            "Un destinatario ya no está disponible. Revisa los dispositivos elegidos; no se puso nada en cola.",
            "Los recursos para compartir están ocupados. Nada se puso en cola. Espera a que termine una transferencia en cola.",
            "La operación no se aceptó. Revisa el estado actual y vuelve a intentarlo.",
            "La visibilidad se guardó, pero no se aplicó por completo. Revisa el estado de detección y Ajustes.",
            "La operación no terminó. Comprueba su estado antes de reintentar.",
        ],
        "pt" => &[
            "O resultado da gravação das definições é incerto. Abra as Definições e repita a gravação pendente.",
            "As definições foram alteradas. Abra as Definições e volte a carregá-las antes de guardar.",
            "As definições requerem atenção. Abra as Definições e verifique o resultado.",
            "A ligação ao serviço perdeu-se. Voltar a ligar cria um novo cliente; selecione novamente os ficheiros não enviados.",
            "O serviço é incompatível ou devolveu um estado inválido. Atualize-o e volte a ligar.",
            "A operação requer recuperação. Não repita o envio.",
            "Não é possível ler os ficheiros selecionados. Verifique o acesso e selecione-os novamente.",
            "Um destinatário já não está disponível. Reveja os dispositivos selecionados; nada entrou na fila.",
            "Os recursos de partilha estão ocupados. Nada entrou na fila. Aguarde que uma transferência em fila termine.",
            "A operação não foi aceite. Reveja o estado atual e tente novamente.",
            "A visibilidade foi guardada, mas não foi totalmente aplicada. Reveja o estado da descoberta e as Definições.",
            "A operação não foi concluída. Verifique o estado antes de tentar novamente.",
        ],
        "de" => &[
            "Das Ergebnis des Speicherns ist ungewiss. Öffnen Sie Einstellungen und wiederholen Sie das ausstehende Speichern.",
            "Die Einstellungen wurden geändert. Öffnen Sie Einstellungen und laden Sie sie vor dem Speichern neu.",
            "Die Einstellungen müssen geprüft werden. Öffnen Sie Einstellungen und prüfen Sie das Ergebnis.",
            "Verbindung zum Dienst verloren. Beim Wiederverbinden entsteht ein neuer Client; wählen Sie ungesendete Dateien erneut aus.",
            "Der Dienst ist inkompatibel oder hat einen ungültigen Zustand gemeldet. Aktualisieren Sie ihn und verbinden Sie sich erneut.",
            "Die Operation erfordert Wiederherstellung. Wiederholen Sie den Sendevorgang nicht.",
            "Die gewählten Dateien können nicht gelesen werden. Prüfen Sie den Zugriff und wählen Sie sie erneut.",
            "Ein Empfänger ist nicht mehr verfügbar. Prüfen Sie die gewählten Geräte; nichts wurde eingereiht.",
            "Freigaberessourcen sind belegt. Nichts wurde eingereiht. Warten Sie, bis eine eingereihte Übertragung endet.",
            "Die Operation wurde nicht angenommen. Prüfen Sie den aktuellen Status und versuchen Sie es erneut.",
            "Die Sichtbarkeit wurde gespeichert, aber nicht vollständig angewendet. Prüfen Sie Erkennungsstatus und Einstellungen.",
            "Die Operation wurde nicht abgeschlossen. Prüfen Sie ihren Status vor einem erneuten Versuch.",
        ],
        "fr" => &[
            "Le résultat de l'enregistrement des paramètres est incertain. Ouvrez Paramètres et réessayez l'enregistrement en attente.",
            "Les paramètres ont changé. Ouvrez Paramètres et rechargez-les avant d'enregistrer.",
            "Les paramètres nécessitent une vérification. Ouvrez Paramètres et examinez le résultat.",
            "Connexion au service perdue. La reconnexion crée un nouveau client ; sélectionnez à nouveau les fichiers non envoyés.",
            "Le service est incompatible ou a renvoyé un état invalide. Mettez-le à jour puis reconnectez-vous.",
            "L'opération nécessite une récupération. Ne répétez pas l'envoi.",
            "Impossible de lire les fichiers sélectionnés. Vérifiez l'accès et sélectionnez-les à nouveau.",
            "Un destinataire n'est plus disponible. Vérifiez les appareils choisis ; rien n'a été mis en attente.",
            "Les ressources de partage sont occupées. Rien n'a été mis en attente. Attendez la fin d'un transfert en attente.",
            "L'opération a été refusée. Vérifiez l'état actuel puis réessayez.",
            "La visibilité a été enregistrée mais pas entièrement appliquée. Vérifiez l'état de découverte et Paramètres.",
            "L'opération n'a pas abouti. Vérifiez son état avant de réessayer.",
        ],
        "it" => &[
            "L'esito del salvataggio delle impostazioni è incerto. Apri Impostazioni e ripeti il salvataggio in sospeso.",
            "Le impostazioni sono cambiate. Apri Impostazioni e ricaricale prima di salvare.",
            "Le impostazioni richiedono attenzione. Apri Impostazioni e verifica il risultato.",
            "Connessione al servizio persa. La riconnessione crea un nuovo client; seleziona di nuovo i file non inviati.",
            "Il servizio non è compatibile o ha restituito uno stato non valido. Aggiornalo e riconnettiti.",
            "L'operazione richiede recupero. Non ripetere l'invio.",
            "Impossibile leggere i file selezionati. Verifica l'accesso e selezionali di nuovo.",
            "Un destinatario non è più disponibile. Controlla i dispositivi scelti; nulla è stato accodato.",
            "Le risorse di condivisione sono occupate. Nulla è stato accodato. Attendi il completamento di un trasferimento in coda.",
            "L'operazione non è stata accettata. Controlla lo stato attuale e riprova.",
            "La visibilità è stata salvata ma non applicata del tutto. Controlla lo stato di rilevamento e Impostazioni.",
            "L'operazione non è terminata. Controlla il suo stato prima di riprovare.",
        ],
        "pl" => &[
            "Wynik zapisania ustawień jest niepewny. Otwórz Ustawienia i ponów oczekujący zapis.",
            "Ustawienia uległy zmianie. Otwórz Ustawienia i wczytaj je ponownie przed zapisaniem.",
            "Ustawienia wymagają uwagi. Otwórz Ustawienia i sprawdź wynik.",
            "Utracono połączenie z usługą. Ponowne połączenie tworzy nowego klienta; wybierz ponownie niewysłane pliki.",
            "Usługa jest niezgodna lub zwróciła nieprawidłowy stan. Zaktualizuj ją i połącz się ponownie.",
            "Operacja wymaga odzyskania. Nie powtarzaj wysyłania.",
            "Nie można odczytać wybranych plików. Sprawdź dostęp i wybierz je ponownie.",
            "Odbiorca nie jest już dostępny. Sprawdź wybrane urządzenia; nic nie dodano do kolejki.",
            "Zasoby udostępniania są zajęte. Nic nie dodano do kolejki. Poczekaj na zakończenie transferu z kolejki.",
            "Operacja nie została zaakceptowana. Sprawdź bieżący stan i spróbuj ponownie.",
            "Widoczność zapisano, ale nie zastosowano w pełni. Sprawdź stan wykrywania i Ustawienia.",
            "Operacja nie została zakończona. Sprawdź jej stan przed ponowieniem.",
        ],
        "hi" => &[
            "सेटिंग सहेजने का परिणाम अनिश्चित है। सेटिंग खोलें और लंबित सेव फिर से आज़माएँ।",
            "सेटिंग बदल गई हैं। सेटिंग खोलें और सहेजने से पहले फिर से लोड करें।",
            "सेटिंग की जाँच आवश्यक है। सेटिंग खोलें और परिणाम देखें।",
            "सेवा से कनेक्शन टूट गया। दोबारा कनेक्ट करने पर नया क्लाइंट बनता है; न भेजी गई फ़ाइलें फिर चुनें।",
            "सेवा असंगत है या उसने अमान्य स्थिति लौटाई। इसे अपडेट करके फिर कनेक्ट करें।",
            "ऑपरेशन को पुनर्प्राप्ति की ज़रूरत है। भेजना दोहराएँ नहीं।",
            "चुनी गई फ़ाइलें पढ़ी नहीं जा सकतीं। पहुँच जाँचें और फिर चुनें।",
            "प्राप्तकर्ता अब उपलब्ध नहीं है। चुने गए डिवाइस देखें; कुछ भी कतार में नहीं गया।",
            "शेयरिंग संसाधन व्यस्त हैं। कुछ भी कतार में नहीं गया। कतार में मौजूद ट्रांसफ़र पूरा होने दें।",
            "ऑपरेशन स्वीकार नहीं हुआ। वर्तमान स्थिति जाँचें और फिर कोशिश करें।",
            "दृश्यता सहेजी गई लेकिन पूरी तरह लागू नहीं हुई। खोज की स्थिति और सेटिंग जाँचें।",
            "ऑपरेशन पूरा नहीं हुआ। फिर कोशिश करने से पहले इसकी स्थिति जाँचें।",
        ],
        "zh-Hant" => &[
            "設定儲存結果尚未確定。請開啟設定並重試待處理的儲存作業。",
            "設定已變更。請開啟設定，儲存前先重新載入。",
            "設定需要處理。請開啟設定並檢查結果。",
            "服務連線已中斷。重新連線會建立新的用戶端；請重新選取尚未傳送的檔案。",
            "服務不相容或傳回無效狀態。請更新服務並重新連線。",
            "操作需要復原。請勿重複傳送。",
            "無法讀取選取的檔案。請檢查存取權限並重新選取。",
            "收件者已無法使用。請檢查選取的裝置；沒有任何項目加入佇列。",
            "分享資源忙碌中。沒有任何項目加入佇列。請等待佇列中的傳輸完成。",
            "操作未被接受。請檢查目前狀態後重試。",
            "可見性已儲存，但未完全套用。請檢查探索狀態和設定。",
            "操作未完成。請檢查狀態後再重試。",
        ],
        "ja" => &[
            "設定の保存結果は不明です。設定を開き、保留中の保存を再試行してください。",
            "設定が変更されました。設定を開き、保存前に再読み込みしてください。",
            "設定の確認が必要です。設定を開いて結果を確認してください。",
            "サービスとの接続が失われました。再接続すると新しいクライアントになります。未送信のファイルを再選択してください。",
            "サービスに互換性がないか、不正な状態を返しました。更新してから再接続してください。",
            "操作の復旧が必要です。送信を繰り返さないでください。",
            "選択したファイルを読み取れません。アクセス権を確認し、再選択してください。",
            "送信先が利用できなくなりました。選択したデバイスを確認してください。何もキューに入りませんでした。",
            "共有リソースが使用中です。何もキューに入りませんでした。キュー内の転送が終わるまでお待ちください。",
            "操作は受け付けられませんでした。現在の状態を確認して再試行してください。",
            "公開状態は保存されましたが、完全には適用されませんでした。検出状態と設定を確認してください。",
            "操作が完了しませんでした。再試行前に状態を確認してください。",
        ],
        "nl" => &[
            "Het resultaat van het opslaan is onzeker. Open Instellingen en probeer de wachtende opslag opnieuw.",
            "Instellingen zijn gewijzigd. Open Instellingen en laad ze opnieuw voordat je opslaat.",
            "Instellingen vereisen aandacht. Open Instellingen en controleer het resultaat.",
            "Verbinding met de dienst verloren. Opnieuw verbinden maakt een nieuwe client; kies niet-verzonden bestanden opnieuw.",
            "De dienst is niet compatibel of leverde een ongeldige status. Werk hem bij en verbind opnieuw.",
            "De bewerking vereist herstel. Herhaal het verzenden niet.",
            "Kan de gekozen bestanden niet lezen. Controleer toegang en selecteer ze opnieuw.",
            "Een ontvanger is niet meer beschikbaar. Controleer de gekozen apparaten; niets is in de wachtrij geplaatst.",
            "Deelbronnen zijn bezet. Niets is in de wachtrij geplaatst. Wacht tot een wachtrijoverdracht is voltooid.",
            "De bewerking is niet geaccepteerd. Controleer de huidige status en probeer opnieuw.",
            "Zichtbaarheid is opgeslagen maar niet volledig toegepast. Controleer detectiestatus en Instellingen.",
            "De bewerking is niet voltooid. Controleer de status voordat je opnieuw probeert.",
        ],
        "sv" => &[
            "Resultatet av att spara inställningar är osäkert. Öppna Inställningar och försök spara det väntande igen.",
            "Inställningarna har ändrats. Öppna Inställningar och läs in dem igen före sparande.",
            "Inställningarna behöver granskas. Öppna Inställningar och kontrollera resultatet.",
            "Anslutningen till tjänsten bröts. Återanslutning skapar en ny klient; välj osända filer igen.",
            "Tjänsten är inkompatibel eller returnerade ett ogiltigt tillstånd. Uppdatera och anslut igen.",
            "Åtgärden kräver återställning. Upprepa inte sändningen.",
            "De valda filerna kan inte läsas. Kontrollera åtkomst och välj dem igen.",
            "En mottagare är inte längre tillgänglig. Kontrollera valda enheter; inget lades i kö.",
            "Delningsresurser är upptagna. Inget lades i kö. Vänta tills en köad överföring är klar.",
            "Åtgärden accepterades inte. Kontrollera aktuellt läge och försök igen.",
            "Synligheten sparades men tillämpades inte helt. Kontrollera upptäcktsstatus och Inställningar.",
            "Åtgärden slutfördes inte. Kontrollera dess status innan du försöker igen.",
        ],
        "tr" => &[
            "Ayarları kaydetmenin sonucu belirsiz. Ayarlar'ı açın ve bekleyen kaydetme işlemini yeniden deneyin.",
            "Ayarlar değişti. Kaydetmeden önce Ayarlar'ı açın ve yeniden yükleyin.",
            "Ayarlar kontrol gerektiriyor. Ayarlar'ı açın ve sonucu inceleyin.",
            "Hizmet bağlantısı kesildi. Yeniden bağlanmak yeni istemci oluşturur; gönderilmemiş dosyaları tekrar seçin.",
            "Hizmet uyumsuz veya geçersiz durum döndürdü. Güncelleyip yeniden bağlanın.",
            "İşlemin kurtarılması gerekiyor. Göndermeyi tekrarlamayın.",
            "Seçilen dosyalar okunamıyor. Erişimi kontrol edin ve yeniden seçin.",
            "Alıcı artık kullanılamıyor. Seçili cihazları kontrol edin; hiçbir şey kuyruğa alınmadı.",
            "Paylaşım kaynakları meşgul. Hiçbir şey kuyruğa alınmadı. Kuyruktaki bir aktarımın bitmesini bekleyin.",
            "İşlem kabul edilmedi. Güncel durumu inceleyin ve tekrar deneyin.",
            "Görünürlük kaydedildi ancak tam uygulanamadı. Keşif durumunu ve Ayarlar'ı kontrol edin.",
            "İşlem tamamlanmadı. Yeniden denemeden önce durumunu kontrol edin.",
        ],
        "ko" => &[
            "설정 저장 결과가 불확실합니다. 설정을 열어 대기 중인 저장을 다시 시도하세요.",
            "설정이 변경되었습니다. 설정을 열고 저장 전에 다시 불러오세요.",
            "설정을 확인해야 합니다. 설정을 열어 결과를 검토하세요.",
            "서비스 연결이 끊어졌습니다. 다시 연결하면 새 클라이언트가 시작됩니다. 미전송 파일을 다시 선택하세요.",
            "서비스가 호환되지 않거나 잘못된 상태를 반환했습니다. 업데이트 후 다시 연결하세요.",
            "작업 복구가 필요합니다. 전송을 반복하지 마세요.",
            "선택한 파일을 읽을 수 없습니다. 접근 권한을 확인하고 다시 선택하세요.",
            "수신자를 더 이상 사용할 수 없습니다. 선택한 기기를 확인하세요. 대기열에 추가된 항목이 없습니다.",
            "공유 리소스가 사용 중입니다. 대기열에 추가된 항목이 없습니다. 대기 중인 전송이 끝날 때까지 기다리세요.",
            "작업이 승인되지 않았습니다. 현재 상태를 확인하고 다시 시도하세요.",
            "공개 상태는 저장되었지만 완전히 적용되지 않았습니다. 검색 상태와 설정을 확인하세요.",
            "작업이 완료되지 않았습니다. 다시 시도하기 전에 상태를 확인하세요.",
        ],
        "bn" => &[
            "সেটিংস সংরক্ষণের ফল অনিশ্চিত। সেটিংস খুলে অপেক্ষমাণ সংরক্ষণ আবার চেষ্টা করুন।",
            "সেটিংস বদলেছে। সংরক্ষণের আগে সেটিংস খুলে আবার লোড করুন।",
            "সেটিংস পরীক্ষা করা দরকার। সেটিংস খুলে ফল দেখুন।",
            "পরিষেবার সংযোগ বিচ্ছিন্ন। আবার সংযোগ করলে নতুন ক্লায়েন্ট তৈরি হয়; না পাঠানো ফাইল আবার বেছে নিন।",
            "পরিষেবা অসামঞ্জস্যপূর্ণ বা অবৈধ অবস্থা ফিরিয়েছে। আপডেট করে আবার সংযোগ করুন।",
            "অপারেশন পুনরুদ্ধার করতে হবে। পাঠানো আবার করবেন না।",
            "নির্বাচিত ফাইল পড়া যায়নি। অ্যাক্সেস পরীক্ষা করে আবার নির্বাচন করুন।",
            "প্রাপক আর উপলব্ধ নন। নির্বাচিত ডিভাইস পরীক্ষা করুন; কিছুই সারিতে যায়নি।",
            "শেয়ারিং সংস্থান ব্যস্ত। কিছুই সারিতে যায়নি। সারির একটি স্থানান্তর শেষ হওয়া পর্যন্ত অপেক্ষা করুন।",
            "অপারেশন গ্রহণ করা হয়নি। বর্তমান অবস্থা দেখে আবার চেষ্টা করুন।",
            "দৃশ্যমানতা সংরক্ষিত হয়েছে কিন্তু পুরোপুরি কার্যকর হয়নি। শনাক্তকরণের অবস্থা ও সেটিংস পরীক্ষা করুন।",
            "অপারেশন শেষ হয়নি। আবার চেষ্টা করার আগে অবস্থা পরীক্ষা করুন।",
        ],
        "th" => &[
            "ผลการบันทึกการตั้งค่ายังไม่แน่นอน เปิดการตั้งค่าและลองบันทึกรายการที่ค้างอีกครั้ง",
            "การตั้งค่าเปลี่ยนไป เปิดการตั้งค่าแล้วโหลดใหม่ก่อนบันทึก",
            "การตั้งค่าต้องตรวจสอบ เปิดการตั้งค่าเพื่อดูผลลัพธ์",
            "ขาดการเชื่อมต่อกับบริการ การเชื่อมต่อใหม่จะสร้างไคลเอนต์ใหม่ เลือกไฟล์ที่ยังไม่ได้ส่งอีกครั้ง",
            "บริการไม่เข้ากันหรือส่งสถานะไม่ถูกต้อง อัปเดตแล้วเชื่อมต่อใหม่",
            "การดำเนินการต้องได้รับการกู้คืน อย่าส่งซ้ำ",
            "อ่านไฟล์ที่เลือกไม่ได้ ตรวจสอบสิทธิ์เข้าถึงแล้วเลือกอีกครั้ง",
            "ผู้รับไม่พร้อมใช้งานแล้ว ตรวจสอบอุปกรณ์ที่เลือก ไม่มีรายการใดเข้าคิว",
            "ทรัพยากรแชร์ไม่ว่าง ไม่มีรายการใดเข้าคิว รอให้การถ่ายโอนที่อยู่ในคิวเสร็จ",
            "ไม่ยอมรับการดำเนินการ ตรวจสอบสถานะปัจจุบันแล้วลองใหม่",
            "บันทึกการมองเห็นแล้ว แต่ยังใช้การเปลี่ยนแปลงได้ไม่ครบ ตรวจสอบสถานะการค้นพบและการตั้งค่า",
            "การดำเนินการไม่เสร็จ ตรวจสอบสถานะก่อนลองใหม่",
        ],
        "id" => &[
            "Hasil menyimpan pengaturan tidak pasti. Buka Pengaturan dan coba simpan kembali yang tertunda.",
            "Pengaturan berubah. Buka Pengaturan dan muat ulang sebelum menyimpan.",
            "Pengaturan perlu ditinjau. Buka Pengaturan dan periksa hasilnya.",
            "Koneksi layanan terputus. Menghubungkan kembali membuat klien baru; pilih ulang file yang belum dikirim.",
            "Layanan tidak kompatibel atau mengembalikan status tidak valid. Perbarui lalu sambungkan kembali.",
            "Operasi perlu pemulihan. Jangan ulangi pengiriman.",
            "File yang dipilih tidak dapat dibaca. Periksa akses lalu pilih kembali.",
            "Penerima sudah tidak tersedia. Periksa perangkat yang dipilih; tidak ada yang masuk antrean.",
            "Sumber daya berbagi sedang sibuk. Tidak ada yang masuk antrean. Tunggu transfer yang antre selesai.",
            "Operasi tidak diterima. Periksa status saat ini lalu coba lagi.",
            "Visibilitas telah disimpan tetapi belum diterapkan sepenuhnya. Periksa status penemuan dan Pengaturan.",
            "Operasi tidak selesai. Periksa statusnya sebelum mencoba lagi.",
        ],
        "fil" => &[
            "Hindi tiyak ang resulta ng pag-save ng mga setting. Buksan ang Settings at subukang muli ang nakabinbing pag-save.",
            "Nagbago ang mga setting. Buksan ang Settings at i-reload bago mag-save.",
            "Kailangang suriin ang mga setting. Buksan ang Settings at tingnan ang resulta.",
            "Nawala ang koneksyon sa serbisyo. Gumagawa ng bagong client ang muling pagkonekta; piliin muli ang mga hindi naipadalang file.",
            "Hindi tugma ang serbisyo o nagbalik ito ng invalid na estado. I-update at kumonekta muli.",
            "Kailangan ng recovery ang operasyon. Huwag ulitin ang pagpapadala.",
            "Hindi mabasa ang mga napiling file. Suriin ang access at piliin muli ang mga ito.",
            "Hindi na available ang isang tatanggap. Suriin ang mga napiling device; walang naipasok sa pila.",
            "Abala ang mga resource ng pag-share. Walang naipasok sa pila. Hintaying matapos ang transfer na nasa pila.",
            "Hindi tinanggap ang operasyon. Suriin ang kasalukuyang estado at subukang muli.",
            "Na-save ang visibility ngunit hindi ganap na nailapat. Suriin ang discovery status at Settings.",
            "Hindi nakumpleto ang operasyon. Suriin ang estado nito bago subukang muli.",
        ],
        "ar" => &[
            "نتيجة حفظ الإعدادات غير مؤكدة. افتح الإعدادات وأعد محاولة الحفظ المعلّق.",
            "تغيّرت الإعدادات. افتح الإعدادات وأعد تحميلها قبل الحفظ.",
            "تحتاج الإعدادات إلى مراجعة. افتح الإعدادات وتحقق من النتيجة.",
            "انقطع الاتصال بالخدمة. تؤدي إعادة الاتصال إلى بدء عميل جديد؛ اختر الملفات غير المرسلة مرة أخرى.",
            "الخدمة غير متوافقة أو أعادت حالة غير صالحة. حدّثها ثم أعد الاتصال.",
            "تحتاج العملية إلى استرداد. لا تكرر الإرسال.",
            "تعذّرت قراءة الملفات المحددة. تحقّق من الوصول وحددها مجددًا.",
            "لم يعد أحد المستلمين متاحًا. راجع الأجهزة المختارة؛ لم يُضف أي شيء إلى قائمة الانتظار.",
            "موارد المشاركة مشغولة. لم يُضف أي شيء إلى قائمة الانتظار. انتظر اكتمال نقل في قائمة الانتظار.",
            "لم تُقبل العملية. راجع الحالة الحالية وحاول مرة أخرى.",
            "حُفظت إمكانية الظهور ولكن لم تُطبّق بالكامل. راجع حالة الاكتشاف والإعدادات.",
            "لم تكتمل العملية. راجع حالتها قبل إعادة المحاولة.",
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
    fn given_all_locales_when_rendering_client_errors_then_sources_are_complete_and_distinct() {
        assert_eq!(KEYS.len(), 12);
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key));
            for locale in LOCALES {
                let value = translated(locale, key).expect("known client error");
                assert!(!value.trim().is_empty(), "{locale}: {key}");
                assert!(!value.contains('{'), "{locale}: {key}");
            }
        }
        for locale in LOCALES {
            let text = |i| translated(locale, KEYS[i]).expect("client error");
            for (a, b) in [(0, 1), (0, 2), (1, 2), (3, 4), (5, 9), (7, 8), (10, 11)] {
                assert_ne!(text(a), text(b), "{locale}: {a}/{b}");
            }
        }
        assert_eq!(translated("en", KEYS[5]), Some(KEYS[5]));
        assert_eq!(translated("pt-BR", KEYS[10]), Some(KEYS[10]));
        assert_eq!(translated("zh-Hans", KEYS[1]), Some(KEYS[1]));
        assert_eq!(translated("unknown", "Unknown outcome"), None);
    }

    #[test]
    fn given_uncertain_or_unsent_results_when_translated_then_critical_meanings_remain_explicit() {
        assert_eq!(
            translated("ru", KEYS[5]),
            Some("Операцию необходимо восстановить. Не повторяйте отправку.")
        );
        assert_eq!(
            translated("pt", KEYS[8]),
            Some("Os recursos de partilha estão ocupados. Nada entrou na fila. Aguarde que uma transferência em fila termine.")
        );
        assert_eq!(
            translated("ar", KEYS[10]),
            Some("حُفظت إمكانية الظهور ولكن لم تُطبّق بالكامل. راجع حالة الاكتشاف والإعدادات.")
        );
    }
}
