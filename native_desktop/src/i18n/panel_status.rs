//! Presentation-only native panel status captions. The original status selection is authoritative.

pub(super) const KEYS: [&str; 12] = [
    "Preview only · service actions disabled",
    "Transfer service unavailable. Check installation, then reconnect.",
    "Service connection lost. Reconnect to review its current state.",
    "Receiving files from the local sender. Abort to release this preparation.",
    "Preparing selection…",
    "Preparation failed. Review service state or select accessible files.",
    "Visibility change outcome is uncertain. Use Retry visibility change.",
    "Visibility was recovered as hidden. Review service settings before enabling discovery.",
    "Transfer service is stopped. Open Settings to repair or start it.",
    "Accepting request…",
    "Declining request…",
    "Cancelling transfer…",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 12] = match locale {
        "ru" => &[
            "Только предпросмотр · действия службы недоступны",
            "Служба передачи недоступна. Проверьте установку и подключитесь снова.",
            "Соединение со службой потеряно. Подключитесь снова, чтобы проверить текущее состояние.",
            "Получение файлов от локального отправителя. Прервите операцию, чтобы освободить подготовку.",
            "Подготовка выбора…",
            "Ошибка подготовки. Проверьте службу или выберите доступные файлы.",
            "Результат изменения видимости неизвестен. Используйте «Повторить изменение видимости».",
            "Видимость восстановлена в скрытом режиме. Проверьте настройки службы, прежде чем включать обнаружение.",
            "Служба передачи остановлена. Откройте настройки, чтобы исправить проблему или запустить её.",
            "Подтверждение запроса…",
            "Отклонение запроса…",
            "Отмена передачи…",
        ],
        "es" => &[
            "Solo vista previa · acciones del servicio deshabilitadas",
            "Servicio de transferencia no disponible. Comprueba la instalación y vuelve a conectar.",
            "Se perdió la conexión con el servicio. Vuelve a conectar para revisar su estado actual.",
            "Recibiendo archivos del remitente local. Cancela para liberar esta preparación.",
            "Preparando la selección…",
            "Falló la preparación. Revisa el estado del servicio o selecciona archivos accesibles.",
            "No se conoce el resultado del cambio de visibilidad. Usa Reintentar cambio de visibilidad.",
            "La visibilidad se recuperó en modo oculto. Revisa los ajustes del servicio antes de activar la detección.",
            "El servicio de transferencia está detenido. Abre Ajustes para repararlo o iniciarlo.",
            "Aceptando solicitud…",
            "Rechazando solicitud…",
            "Cancelando transferencia…",
        ],
        "pt" => &[
            "Apenas pré-visualização · ações do serviço desativadas",
            "Serviço de transferência indisponível. Verifique a instalação e volte a ligar.",
            "Ligação ao serviço perdida. Volte a ligar para verificar o estado atual.",
            "A receber ficheiros do remetente local. Cancele para libertar esta preparação.",
            "A preparar a seleção…",
            "Falha na preparação. Verifique o estado do serviço ou selecione ficheiros acessíveis.",
            "O resultado da alteração da visibilidade é incerto. Utilize Repetir alteração da visibilidade.",
            "A visibilidade foi reposta como oculta. Reveja as definições do serviço antes de ativar a descoberta.",
            "O serviço de transferência está parado. Abra as Definições para o reparar ou iniciar.",
            "A aceitar o pedido…",
            "A recusar o pedido…",
            "A cancelar a transferência…",
        ],
        "de" => &[
            "Nur Vorschau · Dienstaktionen deaktiviert",
            "Übertragungsdienst nicht verfügbar. Prüfen Sie die Installation und verbinden Sie sich erneut.",
            "Die Verbindung zum Dienst ist unterbrochen. Verbinden Sie sich erneut, um den aktuellen Status zu prüfen.",
            "Dateien werden vom lokalen Sender empfangen. Brechen Sie ab, um diese Vorbereitung freizugeben.",
            "Auswahl wird vorbereitet…",
            "Vorbereitung fehlgeschlagen. Prüfen Sie den Dienststatus oder wählen Sie zugängliche Dateien.",
            "Ergebnis der Sichtbarkeitsänderung ungewiss. Verwenden Sie Sichtbarkeitsänderung wiederholen.",
            "Die Sichtbarkeit wurde im verborgenen Zustand wiederhergestellt. Prüfen Sie die Diensteinstellungen vor dem Aktivieren der Erkennung.",
            "Der Übertragungsdienst ist beendet. Öffnen Sie Einstellungen, um ihn zu reparieren oder zu starten.",
            "Anfrage wird angenommen…",
            "Anfrage wird abgelehnt…",
            "Übertragung wird abgebrochen…",
        ],
        "fr" => &[
            "Aperçu uniquement · actions du service désactivées",
            "Service de transfert indisponible. Vérifiez l'installation puis reconnectez-vous.",
            "Connexion au service perdue. Reconnectez-vous pour vérifier son état actuel.",
            "Réception des fichiers de l'expéditeur local. Annulez pour libérer cette préparation.",
            "Préparation de la sélection…",
            "Échec de la préparation. Vérifiez l'état du service ou sélectionnez des fichiers accessibles.",
            "Résultat du changement de visibilité incertain. Utilisez Réessayer le changement de visibilité.",
            "La visibilité a été rétablie en mode masqué. Vérifiez les paramètres avant d'activer la découverte.",
            "Le service de transfert est arrêté. Ouvrez Paramètres pour le réparer ou le démarrer.",
            "Acceptation de la demande…",
            "Refus de la demande…",
            "Annulation du transfert…",
        ],
        "it" => &[
            "Solo anteprima · azioni del servizio disabilitate",
            "Servizio di trasferimento non disponibile. Controlla l'installazione e riconnettiti.",
            "Connessione al servizio persa. Riconnettiti per controllarne lo stato attuale.",
            "Ricezione di file dal mittente locale. Annulla per liberare questa preparazione.",
            "Preparazione della selezione…",
            "Preparazione non riuscita. Controlla lo stato del servizio o scegli file accessibili.",
            "Esito della modifica della visibilità incerto. Usa Riprova modifica visibilità.",
            "La visibilità è stata ripristinata come nascosta. Controlla le impostazioni prima di attivare il rilevamento.",
            "Il servizio di trasferimento è arrestato. Apri Impostazioni per ripararlo o avviarlo.",
            "Accettazione richiesta…",
            "Rifiuto richiesta…",
            "Annullamento trasferimento…",
        ],
        "pl" => &[
            "Tylko podgląd · działania usługi wyłączone",
            "Usługa transferu niedostępna. Sprawdź instalację i połącz się ponownie.",
            "Utracono połączenie z usługą. Połącz się ponownie, aby sprawdzić bieżący stan.",
            "Odbieranie plików od lokalnego nadawcy. Anuluj, aby zwolnić to przygotowanie.",
            "Przygotowywanie wyboru…",
            "Przygotowanie nie powiodło się. Sprawdź stan usługi lub wybierz dostępne pliki.",
            "Wynik zmiany widoczności jest niepewny. Użyj Ponów zmianę widoczności.",
            "Widoczność przywrócono w trybie ukrytym. Sprawdź ustawienia usługi przed włączeniem wykrywania.",
            "Usługa transferu została zatrzymana. Otwórz Ustawienia, aby ją naprawić lub uruchomić.",
            "Akceptowanie żądania…",
            "Odrzucanie żądania…",
            "Anulowanie transferu…",
        ],
        "hi" => &[
            "केवल पूर्वावलोकन · सेवा की कार्रवाइयाँ बंद हैं",
            "ट्रांसफ़र सेवा उपलब्ध नहीं है। इंस्टॉलेशन जाँचें और फिर से कनेक्ट करें।",
            "सेवा का कनेक्शन टूट गया। वर्तमान स्थिति देखने के लिए फिर से कनेक्ट करें।",
            "स्थानीय भेजने वाले से फ़ाइलें प्राप्त हो रही हैं। यह तैयारी मुक्त करने के लिए रद्द करें।",
            "चयन तैयार किया जा रहा है…",
            "तैयारी विफल हुई। सेवा की स्थिति जाँचें या पहुँच योग्य फ़ाइलें चुनें।",
            "दृश्यता बदलाव का परिणाम अनिश्चित है। दृश्यता बदलाव फिर आज़माएँ का उपयोग करें।",
            "दृश्यता छिपी हुई स्थिति में पुनर्स्थापित हुई है। खोज चालू करने से पहले सेवा सेटिंग जाँचें।",
            "ट्रांसफ़र सेवा बंद है। उसे सुधारने या शुरू करने के लिए सेटिंग खोलें।",
            "अनुरोध स्वीकार किया जा रहा है…",
            "अनुरोध अस्वीकार किया जा रहा है…",
            "ट्रांसफ़र रद्द किया जा रहा है…",
        ],
        "zh-Hant" => &[
            "僅預覽 · 服務操作已停用",
            "傳輸服務無法使用。請檢查安裝並重新連線。",
            "服務連線已中斷。請重新連線以查看目前狀態。",
            "正在接收本機傳送者的檔案。請中止以釋放此準備作業。",
            "正在準備選取項目…",
            "準備失敗。請檢查服務狀態或選取可存取的檔案。",
            "可見性變更結果尚未確定。請使用重試可見性變更。",
            "可見性已恢復為隱藏狀態。啟用探索前請檢查服務設定。",
            "傳輸服務已停止。請開啟設定以修復或啟動服務。",
            "正在接受要求…",
            "正在拒絕要求…",
            "正在取消傳輸…",
        ],
        "ja" => &[
            "プレビューのみ · サービス操作は無効です",
            "転送サービスを利用できません。インストールを確認して再接続してください。",
            "サービスとの接続が失われました。再接続して現在の状態を確認してください。",
            "ローカルの送信元からファイルを受信中です。中止してこの準備を解放してください。",
            "選択を準備しています…",
            "準備に失敗しました。サービスの状態を確認するか、アクセスできるファイルを選択してください。",
            "公開状態の変更結果は不明です。公開状態の変更を再試行してください。",
            "公開状態は非表示として復元されました。検出を有効にする前にサービス設定を確認してください。",
            "転送サービスは停止しています。設定を開いて修復または起動してください。",
            "要求を承認中…",
            "要求を拒否中…",
            "転送をキャンセル中…",
        ],
        "nl" => &[
            "Alleen voorbeeld · dienstacties uitgeschakeld",
            "Overdrachtsdienst niet beschikbaar. Controleer de installatie en verbind opnieuw.",
            "Verbinding met de dienst verloren. Verbind opnieuw om de huidige status te controleren.",
            "Bestanden worden ontvangen van de lokale afzender. Breek af om deze voorbereiding vrij te geven.",
            "Selectie voorbereiden…",
            "Voorbereiding mislukt. Controleer de dienststatus of kies toegankelijke bestanden.",
            "Uitkomst van de zichtbaarheidswijziging is onzeker. Gebruik Zichtbaarheidswijziging opnieuw proberen.",
            "Zichtbaarheid is hersteld als verborgen. Controleer de dienstinstellingen voordat je detectie inschakelt.",
            "De overdrachtsdienst is gestopt. Open Instellingen om deze te herstellen of te starten.",
            "Verzoek accepteren…",
            "Verzoek afwijzen…",
            "Overdracht annuleren…",
        ],
        "sv" => &[
            "Endast förhandsvisning · tjänståtgärder inaktiverade",
            "Överföringstjänsten är otillgänglig. Kontrollera installationen och anslut igen.",
            "Anslutningen till tjänsten bröts. Anslut igen för att granska dess aktuella status.",
            "Tar emot filer från den lokala avsändaren. Avbryt för att frigöra denna förberedelse.",
            "Förbereder urval…",
            "Förberedelsen misslyckades. Kontrollera tjänstens status eller välj åtkomliga filer.",
            "Resultatet av ändrad synlighet är osäkert. Använd Försök ändra synlighet igen.",
            "Synligheten återställdes som dold. Granska tjänstinställningarna innan identifiering aktiveras.",
            "Överföringstjänsten är stoppad. Öppna Inställningar för att reparera eller starta den.",
            "Accepterar begäran…",
            "Avböjer begäran…",
            "Avbryter överföring…",
        ],
        "tr" => &[
            "Yalnızca önizleme · hizmet işlemleri devre dışı",
            "Aktarım hizmeti kullanılamıyor. Kurulumu kontrol edip yeniden bağlanın.",
            "Hizmet bağlantısı kesildi. Güncel durumu kontrol etmek için yeniden bağlanın.",
            "Yerel göndericiden dosyalar alınıyor. Bu hazırlığı serbest bırakmak için iptal edin.",
            "Seçim hazırlanıyor…",
            "Hazırlık başarısız oldu. Hizmet durumunu kontrol edin veya erişilebilir dosyalar seçin.",
            "Görünürlük değişikliğinin sonucu belirsiz. Görünürlük değişikliğini yeniden dene seçeneğini kullanın.",
            "Görünürlük gizli olarak geri yüklendi. Keşfi açmadan önce hizmet ayarlarını kontrol edin.",
            "Aktarım hizmeti durduruldu. Onarmak veya başlatmak için Ayarlar'ı açın.",
            "İstek kabul ediliyor…",
            "İstek reddediliyor…",
            "Aktarım iptal ediliyor…",
        ],
        "ko" => &[
            "미리 보기 전용 · 서비스 작업 비활성화",
            "전송 서비스를 사용할 수 없습니다. 설치를 확인한 뒤 다시 연결하세요.",
            "서비스 연결이 끊어졌습니다. 다시 연결하여 현재 상태를 확인하세요.",
            "로컬 발신자로부터 파일을 받는 중입니다. 이 준비 작업을 해제하려면 중단하세요.",
            "선택 항목 준비 중…",
            "준비에 실패했습니다. 서비스 상태를 확인하거나 접근 가능한 파일을 선택하세요.",
            "공개 상태 변경 결과가 불확실합니다. 공개 상태 변경 재시도를 사용하세요.",
            "공개 상태가 숨김으로 복구되었습니다. 검색을 켜기 전에 서비스 설정을 확인하세요.",
            "전송 서비스가 중지되었습니다. 설정을 열어 복구하거나 시작하세요.",
            "요청 수락 중…",
            "요청 거절 중…",
            "전송 취소 중…",
        ],
        "bn" => &[
            "শুধু পূর্বরূপ · পরিষেবার কাজ নিষ্ক্রিয়",
            "স্থানান্তর পরিষেবা অনুপলব্ধ। ইনস্টলেশন পরীক্ষা করে আবার সংযোগ করুন।",
            "পরিষেবার সংযোগ বিচ্ছিন্ন হয়েছে। বর্তমান অবস্থা দেখতে আবার সংযোগ করুন।",
            "স্থানীয় প্রেরকের কাছ থেকে ফাইল গ্রহণ করা হচ্ছে। এই প্রস্তুতি ছাড়তে বাতিল করুন।",
            "নির্বাচন প্রস্তুত হচ্ছে…",
            "প্রস্তুতি ব্যর্থ হয়েছে। পরিষেবার অবস্থা পরীক্ষা করুন বা প্রবেশযোগ্য ফাইল বেছে নিন।",
            "দৃশ্যমানতা পরিবর্তনের ফল অনিশ্চিত। দৃশ্যমানতা পরিবর্তন আবার চেষ্টা করুন।",
            "দৃশ্যমানতা লুকানো অবস্থায় পুনরুদ্ধার হয়েছে। শনাক্তকরণ চালুর আগে পরিষেবার সেটিংস পরীক্ষা করুন।",
            "স্থানান্তর পরিষেবা বন্ধ আছে। মেরামত বা চালু করতে সেটিংস খুলুন।",
            "অনুরোধ গ্রহণ করা হচ্ছে…",
            "অনুরোধ প্রত্যাখ্যান করা হচ্ছে…",
            "স্থানান্তর বাতিল করা হচ্ছে…",
        ],
        "th" => &[
            "ดูตัวอย่างเท่านั้น · ปิดการทำงานของบริการ",
            "บริการถ่ายโอนไม่พร้อมใช้งาน ตรวจสอบการติดตั้งแล้วเชื่อมต่อใหม่",
            "ขาดการเชื่อมต่อกับบริการ เชื่อมต่อใหม่เพื่อตรวจสอบสถานะปัจจุบัน",
            "กำลังรับไฟล์จากผู้ส่งในเครื่อง ยกเลิกเพื่อปล่อยการเตรียมนี้",
            "กำลังเตรียมรายการที่เลือก…",
            "เตรียมข้อมูลไม่สำเร็จ ตรวจสอบสถานะบริการหรือเลือกไฟล์ที่เข้าถึงได้",
            "ผลการเปลี่ยนการมองเห็นยังไม่แน่นอน ใช้ลองเปลี่ยนการมองเห็นอีกครั้ง",
            "คืนค่าการมองเห็นเป็นซ่อนแล้ว ตรวจสอบการตั้งค่าบริการก่อนเปิดการค้นพบ",
            "บริการถ่ายโอนหยุดทำงาน เปิดการตั้งค่าเพื่อซ่อมแซมหรือเริ่มบริการ",
            "กำลังยอมรับคำขอ…",
            "กำลังปฏิเสธคำขอ…",
            "กำลังยกเลิกการถ่ายโอน…",
        ],
        "id" => &[
            "Hanya pratinjau · tindakan layanan dinonaktifkan",
            "Layanan transfer tidak tersedia. Periksa pemasangan lalu sambungkan kembali.",
            "Koneksi ke layanan terputus. Sambungkan kembali untuk memeriksa status saat ini.",
            "Menerima file dari pengirim lokal. Batalkan untuk melepaskan penyiapan ini.",
            "Menyiapkan pilihan…",
            "Penyiapan gagal. Periksa status layanan atau pilih file yang dapat diakses.",
            "Hasil perubahan visibilitas tidak pasti. Gunakan Coba lagi perubahan visibilitas.",
            "Visibilitas dipulihkan sebagai tersembunyi. Periksa pengaturan layanan sebelum mengaktifkan penemuan.",
            "Layanan transfer berhenti. Buka Pengaturan untuk memperbaiki atau memulainya.",
            "Menerima permintaan…",
            "Menolak permintaan…",
            "Membatalkan transfer…",
        ],
        "fil" => &[
            "Preview lang · naka-disable ang mga aksyon ng serbisyo",
            "Hindi available ang transfer service. Suriin ang pag-install at kumonekta muli.",
            "Nawala ang koneksyon sa serbisyo. Kumonekta muli para suriin ang kasalukuyang estado.",
            "Tumatanggap ng mga file mula sa lokal na nagpadala. Kanselahin para bitawan ang paghahandang ito.",
            "Inihahanda ang napili…",
            "Nabigo ang paghahanda. Suriin ang estado ng serbisyo o pumili ng naa-access na file.",
            "Hindi tiyak ang resulta ng pagbabago sa visibility. Gamitin ang Subukang muli ang pagbabago sa visibility.",
            "Naibalik ang visibility bilang nakatago. Suriin ang mga setting ng serbisyo bago paganahin ang discovery.",
            "Huminto ang transfer service. Buksan ang Settings para ayusin o simulan ito.",
            "Tinatanggap ang kahilingan…",
            "Tinatanggihan ang kahilingan…",
            "Kinakansela ang transfer…",
        ],
        "ar" => &[
            "معاينة فقط · إجراءات الخدمة معطّلة",
            "خدمة النقل غير متاحة. تحقّق من التثبيت ثم أعد الاتصال.",
            "انقطع الاتصال بالخدمة. أعد الاتصال لمراجعة حالتها الحالية.",
            "جارٍ استلام الملفات من المرسل المحلي. ألغِ العملية لتحرير هذا الإعداد.",
            "جارٍ تجهيز التحديد…",
            "فشل الإعداد. راجع حالة الخدمة أو اختر ملفات يمكن الوصول إليها.",
            "نتيجة تغيير إمكانية الظهور غير مؤكدة. استخدم إعادة محاولة تغيير إمكانية الظهور.",
            "استُعيدت إمكانية الظهور في الوضع المخفي. راجع إعدادات الخدمة قبل تفعيل الاكتشاف.",
            "خدمة النقل متوقفة. افتح الإعدادات لإصلاحها أو تشغيلها.",
            "جارٍ قبول الطلب…",
            "جارٍ رفض الطلب…",
            "جارٍ إلغاء النقل…",
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
    fn given_twenty_locales_when_displaying_panel_status_then_keys_are_complete_and_disjoint() {
        assert_eq!(KEYS.len(), 12);
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key));
            for locale in LOCALES {
                let label = translated(locale, key).expect("known panel status");
                assert!(!label.trim().is_empty(), "{locale}: {key}");
                assert!(!label.contains('{'), "{locale}: {key}");
            }
        }
        assert_eq!(translated("en", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("pt-BR", KEYS[7]), Some(KEYS[7]));
        assert_eq!(translated("zh-Hans", KEYS[6]), Some(KEYS[6]));
        assert_eq!(translated("unknown", "Missing status"), None);
    }

    #[test]
    fn given_hidden_uncertain_and_pending_states_when_localizing_then_none_claims_completion() {
        for locale in LOCALES {
            let read = |i| translated(locale, KEYS[i]).expect("known state");
            for (first, second) in [(4, 5), (6, 7), (0, 8), (9, 10), (9, 11), (10, 11)] {
                assert_ne!(read(first), read(second), "{locale}: {first}/{second}");
            }
        }
        assert_eq!(
            translated("ru", KEYS[7]),
            Some("Видимость восстановлена в скрытом режиме. Проверьте настройки службы, прежде чем включать обнаружение.")
        );
        assert_eq!(
            translated("pt", KEYS[6]),
            Some("O resultado da alteração da visibilidade é incerto. Utilize Repetir alteração da visibilidade.")
        );
        assert_eq!(translated("ar", KEYS[11]), Some("جارٍ إلغاء النقل…"));
    }
}
