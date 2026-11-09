# Windows: antivirus warnings and code signing / Antivirüs uyarıları ve kod imzalama

## Türkçe

**Neden uyarı veriyor?** Symantec'in "Unproven.Insight" bulgusu bir virüs bulgusu değildir. *Insight*, dosyaları
itibarlarına göre değerlendirir: yeni, az kişide görülmüş ve dijital imzası olmayan her `.exe` "kanıtlanmamış"
sayılır. Programın her yeni derlemesi yeni bir dosya olduğu için bu uyarı her sürümde yeniden çıkabilir.

**Hemen yapılabilecekler (bilgi işlemle):**

1. Symantec Endpoint Protection yöneticisi, dosyayı karantinadan geri alabilir ve SHA-256 özetine ya da program
   klasörüne bir istisna tanımlayabilir.
2. Dosya Broadcom'a yanlış pozitif olarak bildirilebilir: <https://symsubmit.symantec.com>.

Antivirüs korumasını kendiniz kapatmayın; istisnayı bilgi işlem tanımlamalıdır.

**Aynı dosya, aynı özet.** Uygulamanın Dart kodu `data/app.so` dosyasındadır; `uncertainty_app.exe` yalnızca
pencereyi açan küçük bir başlatıcıdır. Kasım 2026'dan beri .exe ve motor DLL'i tekrar üretilebilir biçimde
derlenir (MSVC `/Brepro`): aynı kaynaktan her derleme bayt bayt aynı dosyayı verir. Dolayısıyla .exe'nin SHA-256
özetine tanımlanan bir istisna, başlatıcı değişmediği sürece sonraki sürümlerde de geçerli kalır. Motor DLL'i
program sürümünü ve commit'i içerdiği için her commit'te değişir.

**Kalıcı çözüm: kod imzalama.** Dosyalar güvenilir bir kod imzalama sertifikasıyla (Authenticode) imzalanırsa
itibar sertifikaya bağlanır ve yeni sürümler uyarı vermez. Yollar:

- **Kurumun kendi sertifika altyapısı:** TÜBİTAK'ın bilgisayarlarının güvendiği iç bir kod imzalama sertifikası
  varsa, Symantec bu yayıncıya güvenecek biçimde yapılandırılabilir. En hızlı yol budur.
- **Ticari kod imzalama sertifikası** (OV ya da EV). 2023'ten beri bu sertifikaların anahtarı bir donanım
  belirtecinde ya da bulut imzalama hizmetinde tutulur; bu durumda imzalama bilgi işlemin bilgisayarında
  `signtool` ile ya da hizmetin kendi GitHub adımıyla yapılır.
- **PFX dosyası olarak verilebilen bir sertifika** (örneğin iç altyapıdan): depo ayarlarında
  *Settings → Secrets and variables → Actions* altında iki sır tanımlayın: `WINDOWS_CERTIFICATE` (PFX dosyasının
  base64 hali, örneğin PowerShell'de `[Convert]::ToBase64String([IO.File]::ReadAllBytes('sertifika.pfx'))`) ve
  `WINDOWS_CERTIFICATE_PASSWORD`. `Package` iş akışı bundan sonra Windows paketindeki her `.exe` ve `.dll`
  dosyasını zaman damgasıyla imzalar. Sırlar tanımlı değilse paket imzasız kalır.

Elle imzalamak için (sertifika bilgisayarda ya da belirteçteyse):

```
signtool sign /a /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /d "Uncertainty Calculations" uncertainty_app.exe uncertainty_ffi.dll
```

## English

**Why the warning?** Symantec's "Unproven.Insight" is not a virus detection. *Insight* rates files by reputation:
every new, rarely seen and unsigned `.exe` is "unproven". Each new build of the program is a new file, so the
warning can come back with every version.

**Right away (with the IT department):** the Symantec Endpoint Protection administrator can restore the file
from quarantine and add an exception for its SHA-256 digest or for the program folder; the file can also be
reported as a false positive at <https://symsubmit.symantec.com>. Do not switch the protection off yourself.

**Same file, same digest.** The Dart code is in `data/app.so`; `uncertainty_app.exe` is only the small native
launcher. Since November 2026 the executable and the engine DLL are built reproducibly (MSVC `/Brepro`), so an
exception for the digest of the executable stays valid as long as the launcher does not change. The engine DLL
contains the version and commit of the program and changes with every commit.

**Permanent fix: code signing.** Executables signed with a trusted code-signing certificate (Authenticode) carry
the reputation of the certificate, so new versions are not flagged. Use the organisation's own PKI (and let the
endpoint protection trust that publisher), a commercial OV or EV certificate (keys on a hardware token or a cloud
signing service since 2023), or, for a certificate available as a PFX file, the repository secrets
`WINDOWS_CERTIFICATE` (base64 of the PFX) and `WINDOWS_CERTIFICATE_PASSWORD`: the `Package` workflow then signs
every `.exe` and `.dll` of the Windows package with a timestamp.
