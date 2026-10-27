//! Static sharing settings state captions. Only display changes, no save actions.

pub(super) const KEYS: [&str; 15] = [
    "Sharing settings are unavailable.",
    "Cannot read settings. Restore access, then reload saved values.",
    "Saved settings are damaged or unavailable. Review values and confirm replacement.",
    "Save outcome is uncertain. Retry the same save before making another edit.",
    "Saved, but discovery could not confirm the device name. Reload to check, or restart sharing when idle.",
    "Settings could not be applied. Review the receive folder, device name and port, then restart sharing when idle.",
    "Saved visibility is invalid. Receiving is hidden; choose a visibility and press OK.",
    "Defaults loaded. Press OK to keep your settings.",
    "Existing settings loaded. Press OK to import them; the original file is preserved.",
    "Saved. Service is stopped; start the saved service when ready.",
    "Saved name is not yet confirmed by discovery. Press OK to apply, or reload to check.",
    "Discovery could not confirm the device name. Reload to check, or restart sharing when idle.",
    "Saved values differ from the live policy. Press OK to apply them.",
    "Saved. Visibility and receive folder applied; restart required for the port.",
    "Saved and applied.",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 15] = match locale {
        "ru" => &[
            "Настройки обмена недоступны.",
            "Не удалось прочитать настройки. Восстановите доступ и повторно загрузите сохранённые значения.",
            "Сохранённые настройки повреждены или недоступны. Проверьте значения и подтвердите замену.",
            "Результат сохранения неизвестен. Повторите то же сохранение, прежде чем вносить другие изменения.",
            "Сохранено, но обнаружение не подтвердило имя устройства. Перезагрузите данные или перезапустите обмен после завершения передач.",
            "Не удалось применить настройки. Проверьте папку получения, имя устройства и порт, затем перезапустите обмен после завершения передач.",
            "Сохранённая видимость некорректна. Получение скрыто; выберите видимость и нажмите ОК.",
            "Загружены значения по умолчанию. Нажмите ОК, чтобы сохранить настройки.",
            "Загружены существующие настройки. Нажмите ОК для импорта; исходный файл будет сохранён.",
            "Сохранено. Сервис остановлен; запустите сохранённый сервис, когда будете готовы.",
            "Сохранённое имя пока не подтверждено обнаружением. Нажмите ОК для применения или обновите данные для проверки.",
            "Обнаружение не смогло подтвердить имя устройства. Обновите данные или перезапустите обмен после завершения передач.",
            "Сохранённые значения отличаются от действующей политики. Нажмите ОК, чтобы их применить.",
            "Сохранено. Видимость и папка получения применены; для порта нужен перезапуск.",
            "Сохранено и применено.",
        ],
        "es" => &[
            "Los ajustes para compartir no están disponibles.",
            "No se pueden leer los ajustes. Restablece el acceso y recarga los valores guardados.",
            "Los ajustes guardados están dañados o no están disponibles. Revisa los valores y confirma el reemplazo.",
            "El resultado del guardado es incierto. Repite el mismo guardado antes de hacer otro cambio.",
            "Guardado, pero la detección no pudo confirmar el nombre del dispositivo. Recarga para comprobarlo o reinicia el servicio cuando esté inactivo.",
            "No se pudieron aplicar los ajustes. Revisa la carpeta de recepción, el nombre del dispositivo y el puerto; luego reinicia el servicio cuando esté inactivo.",
            "La visibilidad guardada no es válida. La recepción está oculta; elige una visibilidad y pulsa Aceptar.",
            "Se cargaron los valores predeterminados. Pulsa Aceptar para conservar los ajustes.",
            "Se cargaron los ajustes existentes. Pulsa Aceptar para importarlos; se conserva el archivo original.",
            "Guardado. El servicio está detenido; inícialo cuando estés preparado.",
            "El nombre guardado aún no está confirmado por la detección. Pulsa Aceptar para aplicarlo o recarga para comprobarlo.",
            "La detección no pudo confirmar el nombre del dispositivo. Recarga para comprobarlo o reinicia el servicio cuando esté inactivo.",
            "Los valores guardados difieren de la configuración activa. Pulsa Aceptar para aplicarlos.",
            "Guardado. Se aplicaron la visibilidad y la carpeta de recepción; el puerto requiere reiniciar.",
            "Guardado y aplicado.",
        ],
        "pt" => &[
            "As definições de partilha estão indisponíveis.",
            "Não foi possível ler as definições. Restabeleça o acesso e recarregue os valores guardados.",
            "As definições guardadas estão danificadas ou indisponíveis. Reveja os valores e confirme a substituição.",
            "O resultado da gravação é incerto. Repita a mesma gravação antes de fazer outra alteração.",
            "Guardado, mas a descoberta não confirmou o nome do dispositivo. Recarregue para verificar ou reinicie a partilha quando estiver inativa.",
            "Não foi possível aplicar as definições. Verifique a pasta de receção, o nome do dispositivo e a porta; reinicie a partilha quando estiver inativa.",
            "A visibilidade guardada é inválida. A receção está oculta; escolha a visibilidade e prima OK.",
            "Valores predefinidos carregados. Prima OK para manter as definições.",
            "Definições existentes carregadas. Prima OK para as importar; o ficheiro original será preservado.",
            "Guardado. O serviço está parado; inicie o serviço guardado quando estiver pronto.",
            "O nome guardado ainda não foi confirmado pela descoberta. Prima OK para aplicar ou recarregue para verificar.",
            "A descoberta não conseguiu confirmar o nome do dispositivo. Recarregue para verificar ou reinicie a partilha quando estiver inativa.",
            "Os valores guardados diferem da política ativa. Prima OK para os aplicar.",
            "Guardado. A visibilidade e a pasta de receção foram aplicadas; a porta requer reinício.",
            "Guardado e aplicado.",
        ],
        "de" => &[
            "Freigabeeinstellungen sind nicht verfügbar.",
            "Einstellungen konnten nicht gelesen werden. Stellen Sie den Zugriff wieder her und laden Sie die gespeicherten Werte erneut.",
            "Gespeicherte Einstellungen sind beschädigt oder nicht verfügbar. Prüfen Sie die Werte und bestätigen Sie das Ersetzen.",
            "Das Ergebnis des Speicherns ist ungewiss. Wiederholen Sie denselben Speichervorgang, bevor Sie weitere Änderungen vornehmen.",
            "Gespeichert, aber die Geräteerkennung konnte den Gerätenamen nicht bestätigen. Laden Sie ihn erneut oder starten Sie die Freigabe im Leerlauf neu.",
            "Einstellungen konnten nicht angewendet werden. Prüfen Sie Empfangsordner, Gerätename und Port und starten Sie die Freigabe im Leerlauf neu.",
            "Die gespeicherte Sichtbarkeit ist ungültig. Der Empfang ist verborgen; wählen Sie die Sichtbarkeit und drücken Sie OK.",
            "Standardwerte geladen. Drücken Sie OK, um die Einstellungen zu übernehmen.",
            "Vorhandene Einstellungen geladen. Drücken Sie OK zum Importieren; die Originaldatei bleibt erhalten.",
            "Gespeichert. Der Dienst ist angehalten; starten Sie ihn, wenn Sie bereit sind.",
            "Der gespeicherte Name wurde noch nicht durch die Geräteerkennung bestätigt. Drücken Sie OK zum Anwenden oder laden Sie zur Prüfung neu.",
            "Die Geräteerkennung konnte den Gerätenamen nicht bestätigen. Laden Sie neu oder starten Sie die Freigabe im Leerlauf neu.",
            "Gespeicherte Werte unterscheiden sich von der aktiven Richtlinie. Drücken Sie OK, um sie anzuwenden.",
            "Gespeichert. Sichtbarkeit und Empfangsordner wurden angewendet; der Port erfordert einen Neustart.",
            "Gespeichert und angewendet.",
        ],
        "fr" => &[
            "Les paramètres de partage sont indisponibles.",
            "Impossible de lire les paramètres. Rétablissez l’accès, puis rechargez les valeurs enregistrées.",
            "Les paramètres enregistrés sont endommagés ou indisponibles. Vérifiez les valeurs et confirmez leur remplacement.",
            "Le résultat de l’enregistrement est incertain. Réessayez le même enregistrement avant toute autre modification.",
            "Enregistré, mais la découverte n’a pas confirmé le nom de l’appareil. Rechargez pour vérifier ou redémarrez le partage au repos.",
            "Impossible d’appliquer les paramètres. Vérifiez le dossier de réception, le nom de l’appareil et le port, puis redémarrez le partage au repos.",
            "La visibilité enregistrée est invalide. La réception est masquée ; choisissez la visibilité et appuyez sur OK.",
            "Valeurs par défaut chargées. Appuyez sur OK pour conserver les paramètres.",
            "Paramètres existants chargés. Appuyez sur OK pour les importer ; le fichier d’origine est conservé.",
            "Enregistré. Le service est arrêté ; démarrez-le quand vous êtes prêt.",
            "Le nom enregistré n’est pas encore confirmé par la découverte. Appuyez sur OK pour l’appliquer ou rechargez pour vérifier.",
            "La découverte n’a pas confirmé le nom de l’appareil. Rechargez pour vérifier ou redémarrez le partage au repos.",
            "Les valeurs enregistrées diffèrent de la stratégie active. Appuyez sur OK pour les appliquer.",
            "Enregistré. Visibilité et dossier de réception appliqués ; le port nécessite un redémarrage.",
            "Enregistré et appliqué.",
        ],
        "it" => &[
            "Le impostazioni di condivisione non sono disponibili.",
            "Impossibile leggere le impostazioni. Ripristina l’accesso e ricarica i valori salvati.",
            "Le impostazioni salvate sono danneggiate o non disponibili. Controlla i valori e conferma la sostituzione.",
            "L’esito del salvataggio è incerto. Ripeti lo stesso salvataggio prima di altre modifiche.",
            "Salvato, ma il rilevamento non ha confermato il nome del dispositivo. Ricarica per verificare o riavvia la condivisione quando inattiva.",
            "Impossibile applicare le impostazioni. Controlla cartella di ricezione, nome dispositivo e porta, poi riavvia la condivisione quando inattiva.",
            "La visibilità salvata non è valida. La ricezione è nascosta; scegli la visibilità e premi OK.",
            "Valori predefiniti caricati. Premi OK per mantenere le impostazioni.",
            "Impostazioni esistenti caricate. Premi OK per importarle; il file originale viene conservato.",
            "Salvato. Il servizio è fermo; avvialo quando sei pronto.",
            "Il nome salvato non è ancora confermato dal rilevamento. Premi OK per applicarlo o ricarica per verificare.",
            "Il rilevamento non ha confermato il nome del dispositivo. Ricarica o riavvia la condivisione quando inattiva.",
            "I valori salvati differiscono dalla configurazione attiva. Premi OK per applicarli.",
            "Salvato. Visibilità e cartella di ricezione applicate; la porta richiede un riavvio.",
            "Salvato e applicato.",
        ],
        "pl" => &[
            "Ustawienia udostępniania są niedostępne.",
            "Nie można odczytać ustawień. Przywróć dostęp, a następnie wczytaj zapisane wartości.",
            "Zapisane ustawienia są uszkodzone lub niedostępne. Sprawdź wartości i potwierdź zastąpienie.",
            "Wynik zapisu jest niepewny. Ponów ten sam zapis przed wprowadzeniem kolejnych zmian.",
            "Zapisano, lecz wykrywanie nie potwierdziło nazwy urządzenia. Wczytaj ponownie lub uruchom udostępnianie ponownie, gdy będzie bezczynne.",
            "Nie udało się zastosować ustawień. Sprawdź folder odbioru, nazwę urządzenia i port, po czym uruchom udostępnianie ponownie podczas bezczynności.",
            "Zapisana widoczność jest nieprawidłowa. Odbieranie jest ukryte; wybierz widoczność i naciśnij OK.",
            "Wczytano wartości domyślne. Naciśnij OK, aby zachować ustawienia.",
            "Wczytano istniejące ustawienia. Naciśnij OK, aby je zaimportować; oryginalny plik zostanie zachowany.",
            "Zapisano. Usługa jest zatrzymana; uruchom ją, gdy będziesz gotowy.",
            "Zapisana nazwa nie została jeszcze potwierdzona przez wykrywanie. Naciśnij OK, aby ją zastosować, albo wczytaj ponownie.",
            "Wykrywanie nie potwierdziło nazwy urządzenia. Wczytaj ponownie lub uruchom udostępnianie ponownie, gdy będzie bezczynne.",
            "Zapisane wartości różnią się od aktywnej konfiguracji. Naciśnij OK, aby je zastosować.",
            "Zapisano. Widoczność i folder odbioru zastosowano; port wymaga ponownego uruchomienia.",
            "Zapisano i zastosowano.",
        ],
        "hi" => &[
            "शेयरिंग सेटिंग्स उपलब्ध नहीं हैं।",
            "सेटिंग्स पढ़ी नहीं जा सकीं। पहुँच बहाल करें और सहेजे गए मान फिर लोड करें।",
            "सहेजी गई सेटिंग्स खराब हैं या उपलब्ध नहीं हैं। मान जाँचें और बदलने की पुष्टि करें।",
            "सेव का परिणाम अनिश्चित है। कोई और बदलाव करने से पहले वही सेव दोबारा आज़माएँ।",
            "सेव हुआ, लेकिन खोज से डिवाइस के नाम की पुष्टि नहीं हुई। जाँचने के लिए फिर लोड करें या खाली समय में शेयरिंग फिर शुरू करें।",
            "सेटिंग्स लागू नहीं हो सकीं। प्राप्ति फ़ोल्डर, डिवाइस नाम और पोर्ट जाँचें, फिर खाली समय में शेयरिंग फिर शुरू करें।",
            "सहेजी गई दृश्यता अमान्य है। प्राप्ति छिपी है; दृश्यता चुनें और ठीक है दबाएँ।",
            "डिफ़ॉल्ट मान लोड हुए। सेटिंग्स रखने के लिए ठीक है दबाएँ।",
            "मौजूदा सेटिंग्स लोड हुईं। आयात के लिए ठीक है दबाएँ; मूल फ़ाइल सुरक्षित रखी जाएगी।",
            "सेव हुआ। सेवा बंद है; तैयार होने पर सहेजी गई सेवा शुरू करें।",
            "सहेजे गए नाम की खोज से अभी पुष्टि नहीं हुई है। लागू करने के लिए ठीक है दबाएँ या जाँचने के लिए फिर लोड करें।",
            "खोज से डिवाइस नाम की पुष्टि नहीं हुई। फिर लोड करें या खाली समय में शेयरिंग फिर शुरू करें।",
            "सहेजे गए मान सक्रिय नीति से अलग हैं। उन्हें लागू करने के लिए ठीक है दबाएँ।",
            "सेव हुआ। दृश्यता और प्राप्ति फ़ोल्डर लागू हुए; पोर्ट के लिए पुनः शुरू करना होगा।",
            "सेव किया गया और लागू हुआ।",
        ],
        "zh-Hant" => &[
            "分享設定無法使用。",
            "無法讀取設定。請恢復存取權後重新載入已儲存的值。",
            "已儲存的設定損毀或無法使用。請檢查數值並確認取代。",
            "儲存結果尚未確定。在進行其他修改前，請重試相同的儲存操作。",
            "已儲存，但探索功能無法確認裝置名稱。請重新載入以檢查，或在閒置時重新啟動分享。",
            "無法套用設定。請檢查接收資料夾、裝置名稱和連接埠，再於閒置時重新啟動分享。",
            "已儲存的可見性設定無效。接收功能已隱藏；請選擇可見性並按下確定。",
            "已載入預設值。請按確定以保留設定。",
            "已載入現有設定。按確定即可匯入；原始檔案會保留。",
            "已儲存。服務已停止；準備好時再啟動已儲存的服務。",
            "探索功能尚未確認已儲存的名稱。按確定以套用，或重新載入以檢查。",
            "探索功能無法確認裝置名稱。請重新載入以檢查，或在閒置時重新啟動分享。",
            "已儲存的值與目前生效的政策不同。請按確定以套用。",
            "已儲存。可見性與接收資料夾已套用；連接埠需要重新啟動。",
            "已儲存並套用。",
        ],
        "ja" => &[
            "共有設定を利用できません。",
            "設定を読み取れません。アクセスを復旧して、保存済みの値を再読み込みしてください。",
            "保存された設定が破損しているか利用できません。値を確認して置き換えを承認してください。",
            "保存結果を確認できません。別の変更をする前に、同じ保存操作を再試行してください。",
            "保存されましたが、検出でデバイス名を確認できませんでした。再読み込みするか、待機中に共有を再起動してください。",
            "設定を適用できませんでした。受信フォルダー、デバイス名、ポートを確認し、待機中に共有を再起動してください。",
            "保存された公開範囲は無効です。受信は非表示です。公開範囲を選んで OK を押してください。",
            "既定値を読み込みました。設定を保持するには OK を押してください。",
            "既存の設定を読み込みました。OK を押すとインポートします。元のファイルは保持されます。",
            "保存しました。サービスは停止しています。準備ができたらサービスを起動してください。",
            "保存された名前は検出でまだ確認されていません。OK で適用するか、再読み込みして確認してください。",
            "検出でデバイス名を確認できませんでした。再読み込みするか、待機中に共有を再起動してください。",
            "保存された値は現在有効な設定と異なります。OK を押して適用してください。",
            "保存しました。公開範囲と受信フォルダーは適用済みです。ポートには再起動が必要です。",
            "保存して適用しました。",
        ],
        "nl" => &[
            "De instellingen voor delen zijn niet beschikbaar.",
            "Instellingen konden niet worden gelezen. Herstel de toegang en laad opgeslagen waarden opnieuw.",
            "Opgeslagen instellingen zijn beschadigd of niet beschikbaar. Controleer de waarden en bevestig vervanging.",
            "Het resultaat van opslaan is onzeker. Probeer exact dezelfde opslag opnieuw voordat u iets wijzigt.",
            "Opgeslagen, maar apparaatdetectie kon de apparaatnaam niet bevestigen. Laad opnieuw of herstart delen wanneer er geen activiteit is.",
            "Instellingen konden niet worden toegepast. Controleer ontvangstmap, apparaatnaam en poort; herstart delen wanneer er geen activiteit is.",
            "Opgeslagen zichtbaarheid is ongeldig. Ontvangst is verborgen; kies de zichtbaarheid en druk op OK.",
            "Standaardwaarden geladen. Druk op OK om de instellingen te behouden.",
            "Bestaande instellingen geladen. Druk op OK om ze te importeren; het oorspronkelijke bestand blijft behouden.",
            "Opgeslagen. De service is gestopt; start deze wanneer u klaar bent.",
            "De opgeslagen naam is nog niet bevestigd door apparaatdetectie. Druk op OK om toe te passen of laad opnieuw ter controle.",
            "Apparaatdetectie kon de apparaatnaam niet bevestigen. Laad opnieuw of herstart delen wanneer er geen activiteit is.",
            "Opgeslagen waarden verschillen van het actieve beleid. Druk op OK om ze toe te passen.",
            "Opgeslagen. Zichtbaarheid en ontvangstmap toegepast; de poort vereist opnieuw starten.",
            "Opgeslagen en toegepast.",
        ],
        "sv" => &[
            "Delningsinställningarna är inte tillgängliga.",
            "Kunde inte läsa inställningarna. Återställ åtkomsten och läs in sparade värden igen.",
            "Sparade inställningar är skadade eller otillgängliga. Granska värdena och bekräfta att de ska ersättas.",
            "Det är osäkert om sparandet lyckades. Upprepa samma sparande innan du gör fler ändringar.",
            "Sparat, men upptäckten kunde inte bekräfta enhetens namn. Läs in igen eller starta om delning när den är inaktiv.",
            "Inställningarna kunde inte tillämpas. Kontrollera mottagningsmapp, enhetsnamn och port och starta om delning när den är inaktiv.",
            "Sparad synlighet är ogiltig. Mottagning är dold; välj synlighet och tryck på OK.",
            "Standardvärden har lästs in. Tryck på OK för att behålla inställningarna.",
            "Befintliga inställningar har lästs in. Tryck på OK för att importera dem; originalfilen bevaras.",
            "Sparat. Tjänsten är stoppad; starta den när du är redo.",
            "Det sparade namnet är ännu inte bekräftat av upptäckten. Tryck på OK för att tillämpa eller läs in igen.",
            "Upptäckten kunde inte bekräfta enhetens namn. Läs in igen eller starta om delning när den är inaktiv.",
            "Sparade värden skiljer sig från den aktiva policyn. Tryck på OK för att tillämpa dem.",
            "Sparat. Synlighet och mottagningsmapp tillämpades; porten kräver omstart.",
            "Sparat och tillämpat.",
        ],
        "tr" => &[
            "Paylaşım ayarları kullanılamıyor.",
            "Ayarlar okunamadı. Erişimi geri yükleyin ve kayıtlı değerleri yeniden yükleyin.",
            "Kayıtlı ayarlar bozuk veya kullanılamıyor. Değerleri gözden geçirip değiştirmeyi onaylayın.",
            "Kaydetme sonucu belirsiz. Başka bir değişiklik yapmadan aynı kaydetme işlemini yeniden deneyin.",
            "Kaydedildi ancak keşif cihaz adını doğrulayamadı. Yeniden yükleyin veya paylaşım boşta iken yeniden başlatın.",
            "Ayarlar uygulanamadı. Alma klasörünü, cihaz adını ve bağlantı noktasını kontrol edin; paylaşımı boşta iken yeniden başlatın.",
            "Kayıtlı görünürlük geçersiz. Alma gizlendi; görünürlüğü seçip Tamam'a basın.",
            "Varsayılan değerler yüklendi. Ayarları korumak için Tamam'a basın.",
            "Mevcut ayarlar yüklendi. İçeri almak için Tamam'a basın; özgün dosya korunur.",
            "Kaydedildi. Hizmet durduruldu; hazır olduğunuzda başlatın.",
            "Kayıtlı ad keşif tarafından henüz doğrulanmadı. Uygulamak için Tamam'a basın veya kontrol için yeniden yükleyin.",
            "Keşif cihaz adını doğrulayamadı. Yeniden yükleyin veya paylaşımı boşta iken yeniden başlatın.",
            "Kayıtlı değerler etkin ilkeden farklı. Uygulamak için Tamam'a basın.",
            "Kaydedildi. Görünürlük ve alma klasörü uygulandı; bağlantı noktası için yeniden başlatma gerekir.",
            "Kaydedildi ve uygulandı.",
        ],
        "ko" => &[
            "공유 설정을 사용할 수 없습니다.",
            "설정을 읽을 수 없습니다. 접근 권한을 복구하고 저장된 값을 다시 불러오세요.",
            "저장된 설정이 손상되었거나 사용할 수 없습니다. 값을 검토하고 교체를 확인하세요.",
            "저장 결과를 확인할 수 없습니다. 다른 변경을 하기 전에 같은 저장 작업을 다시 시도하세요.",
            "저장되었지만 검색에서 기기 이름을 확인하지 못했습니다. 다시 불러오거나 대기 중에 공유를 재시작하세요.",
            "설정을 적용하지 못했습니다. 수신 폴더, 기기 이름, 포트를 확인하고 대기 중에 공유를 재시작하세요.",
            "저장된 공개 상태가 유효하지 않습니다. 수신이 숨겨졌습니다. 공개 상태를 선택하고 확인을 누르세요.",
            "기본값을 불러왔습니다. 설정을 유지하려면 확인을 누르세요.",
            "기존 설정을 불러왔습니다. 가져오려면 확인을 누르세요. 원본 파일은 유지됩니다.",
            "저장되었습니다. 서비스가 중지되었습니다. 준비되면 서비스를 시작하세요.",
            "저장된 이름이 아직 검색에서 확인되지 않았습니다. 적용하려면 확인을 누르거나 다시 불러와 확인하세요.",
            "검색에서 기기 이름을 확인하지 못했습니다. 다시 불러오거나 대기 중에 공유를 재시작하세요.",
            "저장된 값이 현재 활성 정책과 다릅니다. 적용하려면 확인을 누르세요.",
            "저장되었습니다. 공개 상태와 수신 폴더는 적용되었으며 포트는 재시작이 필요합니다.",
            "저장하고 적용했습니다.",
        ],
        "bn" => &[
            "শেয়ারিং সেটিংস পাওয়া যাচ্ছে না।",
            "সেটিংস পড়া যায়নি। প্রবেশাধিকার ফিরিয়ে এনে সংরক্ষিত মান আবার লোড করুন।",
            "সংরক্ষিত সেটিংস নষ্ট হয়েছে বা পাওয়া যাচ্ছে না। মান যাচাই করে প্রতিস্থাপন নিশ্চিত করুন।",
            "সংরক্ষণের ফল নিশ্চিত নয়। অন্য পরিবর্তনের আগে একই সংরক্ষণ আবার চেষ্টা করুন।",
            "সংরক্ষিত হয়েছে, তবে অনুসন্ধান ডিভাইসের নাম নিশ্চিত করতে পারেনি। আবার লোড করুন বা অব্যবহৃত অবস্থায় শেয়ারিং পুনরায় চালু করুন।",
            "সেটিংস প্রয়োগ করা যায়নি। গ্রহণের ফোল্ডার, ডিভাইস নাম ও পোর্ট যাচাই করে অব্যবহৃত অবস্থায় শেয়ারিং পুনরায় চালু করুন।",
            "সংরক্ষিত দৃশ্যমানতা অবৈধ। গ্রহণ আড়াল করা হয়েছে; দৃশ্যমানতা বেছে ঠিক আছে চাপুন।",
            "ডিফল্ট মান লোড হয়েছে। সেটিংস রাখতে ঠিক আছে চাপুন।",
            "বিদ্যমান সেটিংস লোড হয়েছে। আমদানি করতে ঠিক আছে চাপুন; মূল ফাইল রাখা হবে।",
            "সংরক্ষিত হয়েছে। সেবা বন্ধ রয়েছে; প্রস্তুত হলে চালু করুন।",
            "সংরক্ষিত নাম এখনও অনুসন্ধান নিশ্চিত করেনি। প্রয়োগে ঠিক আছে চাপুন বা যাচাই করতে আবার লোড করুন।",
            "অনুসন্ধান ডিভাইসের নাম নিশ্চিত করতে পারেনি। আবার লোড করুন বা অব্যবহৃত অবস্থায় শেয়ারিং পুনরায় চালু করুন।",
            "সংরক্ষিত মান সক্রিয় নীতির থেকে আলাদা। প্রয়োগ করতে ঠিক আছে চাপুন।",
            "সংরক্ষিত হয়েছে। দৃশ্যমানতা ও গ্রহণ ফোল্ডার প্রয়োগ হয়েছে; পোর্টের জন্য পুনরায় চালু করতে হবে।",
            "সংরক্ষিত ও প্রয়োগ হয়েছে।",
        ],
        "th" => &[
            "ไม่สามารถใช้การตั้งค่าการแชร์ได้",
            "อ่านการตั้งค่าไม่ได้ โปรดคืนสิทธิ์การเข้าถึงแล้วโหลดค่าที่บันทึกไว้อีกครั้ง",
            "การตั้งค่าที่บันทึกไว้เสียหายหรือไม่พร้อมใช้งาน ตรวจสอบค่าและยืนยันการแทนที่",
            "ผลการบันทึกยังไม่แน่นอน ลองบันทึกเดิมอีกครั้งก่อนแก้ไขอย่างอื่น",
            "บันทึกแล้วแต่การค้นหาไม่ยืนยันชื่ออุปกรณ์ โหลดใหม่เพื่อตรวจสอบหรือเริ่มการแชร์ใหม่เมื่อไม่มีการใช้งาน",
            "ใช้การตั้งค่าไม่ได้ ตรวจสอบโฟลเดอร์รับ ชื่ออุปกรณ์ และพอร์ต แล้วเริ่มการแชร์ใหม่เมื่อไม่มีการใช้งาน",
            "การมองเห็นที่บันทึกไว้ไม่ถูกต้อง การรับถูกซ่อน เลือกการมองเห็นแล้วกดตกลง",
            "โหลดค่าเริ่มต้นแล้ว กดตกลงเพื่อเก็บการตั้งค่า",
            "โหลดการตั้งค่าที่มีอยู่แล้ว กดตกลงเพื่อนำเข้า โดยเก็บไฟล์เดิมไว้",
            "บันทึกแล้ว บริการหยุดอยู่ เริ่มบริการเมื่อพร้อม",
            "ชื่อที่บันทึกไว้ยังไม่ยืนยันผ่านการค้นหา กดตกลงเพื่อใช้หรือโหลดใหม่เพื่อตรวจสอบ",
            "การค้นหาไม่ยืนยันชื่ออุปกรณ์ โหลดใหม่หรือเริ่มการแชร์ใหม่เมื่อไม่มีการใช้งาน",
            "ค่าที่บันทึกไว้ต่างจากนโยบายที่ใช้อยู่ กดตกลงเพื่อใช้ค่า",
            "บันทึกแล้ว ใช้การมองเห็นและโฟลเดอร์รับแล้ว พอร์ตต้องเริ่มใหม่",
            "บันทึกและใช้งานแล้ว",
        ],
        "id" => &[
            "Pengaturan berbagi tidak tersedia.",
            "Tidak dapat membaca pengaturan. Pulihkan akses, lalu muat ulang nilai tersimpan.",
            "Pengaturan tersimpan rusak atau tidak tersedia. Tinjau nilai dan konfirmasi penggantian.",
            "Hasil penyimpanan belum pasti. Ulangi penyimpanan yang sama sebelum membuat perubahan lain.",
            "Tersimpan, tetapi penemuan tidak dapat mengonfirmasi nama perangkat. Muat ulang untuk memeriksa atau mulai ulang berbagi saat menganggur.",
            "Pengaturan tidak dapat diterapkan. Periksa folder penerimaan, nama perangkat, dan port; lalu mulai ulang berbagi saat menganggur.",
            "Visibilitas tersimpan tidak valid. Penerimaan disembunyikan; pilih visibilitas dan tekan OK.",
            "Nilai bawaan dimuat. Tekan OK untuk mempertahankan pengaturan.",
            "Pengaturan yang ada dimuat. Tekan OK untuk mengimpornya; file asli dipertahankan.",
            "Tersimpan. Layanan berhenti; mulai layanan saat siap.",
            "Nama tersimpan belum dikonfirmasi oleh penemuan. Tekan OK untuk menerapkan atau muat ulang untuk memeriksa.",
            "Penemuan tidak dapat mengonfirmasi nama perangkat. Muat ulang atau mulai ulang berbagi saat menganggur.",
            "Nilai tersimpan berbeda dari kebijakan aktif. Tekan OK untuk menerapkannya.",
            "Tersimpan. Visibilitas dan folder penerimaan diterapkan; port memerlukan mulai ulang.",
            "Tersimpan dan diterapkan.",
        ],
        "fil" => &[
            "Hindi available ang mga setting sa pagbabahagi.",
            "Hindi mabasa ang mga setting. Ibalik ang access at i-reload ang mga naka-save na halaga.",
            "Nasira o hindi available ang mga naka-save na setting. Suriin ang mga halaga at kumpirmahin ang pagpapalit.",
            "Hindi tiyak ang resulta ng pag-save. Ulitin ang parehong pag-save bago gumawa ng ibang pagbabago.",
            "Na-save, pero hindi makumpirma ng discovery ang pangalan ng device. I-reload para suriin o i-restart ang sharing kapag walang gawain.",
            "Hindi mailapat ang mga setting. Suriin ang folder ng pagtanggap, pangalan ng device at port; i-restart ang sharing kapag walang gawain.",
            "Hindi wasto ang naka-save na visibility. Nakatago ang pagtanggap; piliin ang visibility at pindutin ang OK.",
            "Na-load ang mga default. Pindutin ang OK upang panatilihin ang mga setting.",
            "Na-load ang mga kasalukuyang setting. Pindutin ang OK upang i-import; mananatili ang orihinal na file.",
            "Na-save. Nakahinto ang serbisyo; simulan ito kapag handa na.",
            "Hindi pa nakukumpirma ng discovery ang naka-save na pangalan. Pindutin ang OK upang ilapat o i-reload upang suriin.",
            "Hindi nakumpirma ng discovery ang pangalan ng device. I-reload o i-restart ang sharing kapag walang gawain.",
            "Iba ang mga naka-save na halaga sa aktibong patakaran. Pindutin ang OK upang ilapat.",
            "Na-save. Nailapat ang visibility at folder ng pagtanggap; kailangan ng restart para sa port.",
            "Na-save at nailapat.",
        ],
        "ar" => &[
            "إعدادات المشاركة غير متاحة.",
            "تعذّرت قراءة الإعدادات. استعد الوصول ثم أعد تحميل القيم المحفوظة.",
            "الإعدادات المحفوظة تالفة أو غير متاحة. راجع القيم وأكّد الاستبدال.",
            "نتيجة الحفظ غير مؤكدة. أعد محاولة الحفظ نفسها قبل إجراء تعديل آخر.",
            "تم الحفظ، لكن الاكتشاف لم يؤكد اسم الجهاز. أعد التحميل للتحقق أو أعد تشغيل المشاركة عندما تكون خاملة.",
            "تعذّر تطبيق الإعدادات. راجع مجلد الاستلام واسم الجهاز والمنفذ، ثم أعد تشغيل المشاركة عندما تكون خاملة.",
            "إعداد الظهور المحفوظ غير صالح. الاستلام مخفي؛ اختر الظهور واضغط موافق.",
            "تم تحميل القيم الافتراضية. اضغط موافق للاحتفاظ بالإعدادات.",
            "تم تحميل الإعدادات الموجودة. اضغط موافق لاستيرادها؛ سيُحتفظ بالملف الأصلي.",
            "تم الحفظ. الخدمة متوقفة؛ شغّلها عندما تكون مستعدًا.",
            "لم يؤكد الاكتشاف الاسم المحفوظ بعد. اضغط موافق للتطبيق أو أعد التحميل للتحقق.",
            "لم يؤكد الاكتشاف اسم الجهاز. أعد التحميل للتحقق أو أعد تشغيل المشاركة عندما تكون خاملة.",
            "القيم المحفوظة تختلف عن السياسة النشطة. اضغط موافق لتطبيقها.",
            "تم الحفظ. طُبّق إعداد الظهور ومجلد الاستلام؛ يلزم إعادة تشغيل للمنفذ.",
            "تم الحفظ والتطبيق.",
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
    fn settings_status_catalog_is_complete_unique_and_english_fallback() {
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key), "duplicate status {key}");
            assert_eq!(translated("en", key), Some(*key));
            for locale in LOCALES {
                let text = translated(locale, key).expect("known settings status");
                assert!(!text.trim().is_empty(), "{locale}: {key}");
                assert!(!text.contains('{'), "{locale}: {key}");
            }
        }
        assert_eq!(
            translated("pt-BR", "Saved and applied."),
            Some("Saved and applied.")
        );
        assert_eq!(
            translated("zh-Hans", "Saved and applied."),
            Some("Saved and applied.")
        );
        assert_eq!(
            translated("unknown", "Saved and applied."),
            Some("Saved and applied.")
        );
        assert_eq!(translated("ru", "Peer name"), None);
    }

    #[test]
    fn settings_uncertain_or_corrupt_can_never_read_as_saved_and_applied() {
        for locale in LOCALES {
            let success = translated(locale, "Saved and applied.").expect("success key");
            for key in KEYS.iter().filter(|key| **key != "Saved and applied.") {
                assert_ne!(translated(locale, key), Some(success), "{locale}: {key}");
            }
            let uncertain = translated(locale, KEYS[3]).expect("uncertain");
            assert_ne!(uncertain, translated(locale, KEYS[2]).expect("damaged"));
            assert_ne!(
                uncertain,
                translated(locale, KEYS[5]).expect("apply failed")
            );
            assert_ne!(uncertain, translated(locale, KEYS[9]).expect("stopped"));
            assert_ne!(
                success,
                translated(locale, KEYS[13]).expect("restart pending")
            );
        }
        assert_eq!(
            translated("ru", "Saved and applied."),
            Some("Сохранено и применено.")
        );
        assert_eq!(
            translated("pt", "Saved and applied."),
            Some("Guardado e aplicado.")
        );
        assert_eq!(
            translated(
                "pt",
                "Save outcome is uncertain. Retry the same save before making another edit."
            ),
            Some(
                "O resultado da gravação é incerto. Repita a mesma gravação antes de fazer outra alteração."
            )
        );
    }
}
