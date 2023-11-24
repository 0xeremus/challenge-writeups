### FLARE ON 2019 WRITEUP

#### Problem:  Flare_on 2019 Challenge 1
##### Solution: Kitteh_save_galixy@flare-on.com
##### Tool: Dnspy

##### _Explanation:_

Disassemble .NET file:

Stage 1 Code: 

        private void FireButton_Click(object sender, EventArgs e)
        {
	        if (this.codeTextBox.Text == "RAINBOW") <- this is the code
	        {
		        this.fireButton.Visible = false;
		        this.codeTextBox.Visible = false;
		        this.armingCodeLabel.Visible = false;
		        this.invalidWeaponLabel.Visible = false;
		        this.WeaponCode = this.codeTextBox.Text;
		        this.victoryAnimationTimer.Start();
		        return;
	        }
	        this.invalidWeaponLabel.Visible = true;
	        this.codeTextBox.Text = "";
        }

Stage 2 code: 

        private void FireButton_Click(object sender, EventArgs e)
        {
	        if (this.isValidWeaponCode(this.codeTextBox.Text)) // Calls isValidWeaponCode
	        
        private bool isValidWeaponCode(string s)
        {
	        char[] array = s.ToCharArray();
	        int length = s.Length;
	        for (int i = 0; i < length; i++)
	        {
		        char[] array2 = array;
		        int num = i;
		        array2[num] ^= 'A';
	        }
	        return array.SequenceEqual(new char[]  // This array is xor'd with 'A' thats the code
	        {
		        '\u0003',
		        ' ',
		        '&',
		        '$',
		        '-',
		        '\u001e',
		        '\u0002',
		        ' ',
		        '/',
		        '/',
		        '.',
		        '/'
	        });
        }

##### _Solution Script:_

        x = ['\x03', ' ', '&', '$', '-', '\x1e', '\x02', ' ', '/', '/', '.', '/']
        code = [chr(ord('A') ^ ord(x[i])) for i in range(len(x))]
        ''.join(code)
        'Bagel_Cannon'

#### Problem:  Flare_on 2019 Challenge 2 Overlong
##### Solution: I_a_M_t_h_e_e_n_C_o_D_i_n_g@flare-on.com
##### Tool: My beloved radare2

##### _Explanation:_

Like an idiot I started trying to break the encoding by reversing the function 
that seemed to decode it.

Actual Solution: Call strings on the binary should have taken 30seconds took 45mins (half of which was figuring out how to use radare on windows) :(

    [Strings]
    Num Paddr      Vaddr      Len Size Section  Type  String
    000 0x0000004d 0x0000004d  44  45 () ascii !This program cannot be run in DOS mode.\r\r\n$
    001 0x000000b0 0x000000b0   4   5 () ascii Rich
    002 0x000001c0 0x000001c0   5   6 () ascii .text
    003 0x000001e7 0x000001e7   7   8 () ascii `.rdata
    004 0x0000020f 0x0000020f   6   7 () ascii @.data
    005 0x00000238 0x00000238   6   7 () ascii .reloc
    006 0x00000470 0x00401070   4   6 (.text)  utf8 ?\v╚ëM blocks=Basic Latin,Latin Extended-B
    007 0x000004d5 0x004010d5   4   6 (.text)  utf8 ?\v╤ëU blocks=Basic Latin,Cyrillic
    008 0x00000521 0x00401121   4   6 (.text)  utf8 ?\vIE
    009 0x000005cb 0x004011cb   4   5 (.text) ascii h\b @
    010 0x000005ed 0x004011ed   5  12 (.text) utf16le jhΣÇ░Φ┤Çτ▓ò blocks=Basic Latin,CJK Unified Ideographs Extension A,CJK Unified Ideographs
    011 0x00000808 0x00402008  68 176 (.rdata)  utf8 I never broke the encoding: I_a_M_t_h_e_e_n_C_o_D_i_n_g@flare-on.com
    012 0x000008ea 0x004020ea  11  12 (.rdata) ascii MessageBoxA
    013 0x000008f6 0x004020f6  10  11 (.rdata) ascii USER32.dll
    014 0x00000a00 0x00403000   6   7 (.data) ascii Output

I was all like surely I_a_M_t_h_e_e_n_C_o_D_i_n_g@flare-on.com this is not it... it was.

#### Problem:  Flare_on 2019 Challenge 3 Flarebear.apk 
##### Solution: th4t_was_be4rly_a_chall3nge@flare-on.com 
##### Tool: jd-gui

##### _Explanation:_

Steps: 
1. Set up Android_x86 in a vm
2. Run dex2jar on apk
3. decompile it is jd-gui
4. read the FlareBearActivity class

In the com/fireeye/FlareBearActivity.class there is a function called decrypt 

    public final byte[] decrypt(@NotNull String var1, @NotNull byte[] var2) {
      Intrinsics.checkParameterIsNotNull(var1, "password");
      Intrinsics.checkParameterIsNotNull(var2, "data");
      Charset var3 = Charset.forName("UTF-8");
      Intrinsics.checkExpressionValueIsNotNull(var3, "Charset.forName(charsetName)");
      byte[] var10 = "pawsitive_vibes!".getBytes(var3);
      Intrinsics.checkExpressionValueIsNotNull(var10, "(this as java.lang.String).getBytes(charset)");
      IvParameterSpec var11 = new IvParameterSpec(var10);
      char[] var5 = var1.toCharArray();
      Intrinsics.checkExpressionValueIsNotNull(var5, "(this as java.lang.String).toCharArray()");
      Charset var4 = Charset.forName("UTF-8");
      Intrinsics.checkExpressionValueIsNotNull(var4, "Charset.forName(charsetName)");
      byte[] var12 = "NaClNaClNaCl".getBytes(var4);
      Intrinsics.checkExpressionValueIsNotNull(var12, "(this as java.lang.String).getBytes(charset)");
      PBEKeySpec var6 = new PBEKeySpec(var5, var12, 1234, 256);
      SecretKey var7 = SecretKeyFactory.getInstance("PBKDF2WithHmacSHA1").generateSecret((KeySpec)var6);
      Intrinsics.checkExpressionValueIsNotNull(var7, "secretKeyFactory.generateSecret(pbKeySpec)");
      SecretKeySpec var8 = new SecretKeySpec(var7.getEncoded(), "AES");
      Cipher var13 = Cipher.getInstance("AES/CBC/PKCS7Padding");
      var13.init(2, (Key)var8, (AlgorithmParameterSpec)var11);
      byte[] var9 = var13.doFinal(var2);
      Intrinsics.checkExpressionValueIsNotNull(var9, "cipher.doFinal(data)");
      return var9;
   }

   From this I see:
   Encryption function =  AES/CBC/PKCS7Padding
   Salt = "NaClNaClNaCl"
   IV = pawsitive_vibes!

   There is obviously a lot going on here. I also found that setMood calls the method isHappy() and isEstactic() which triggers the code path below.
   		
   		setMood() -> danceWithFlag() -> getPassword() -> decrypt()

   	so we can just make the bear happy and then estatic and not have to reverse much at all.

    public final boolean isEcstatic()
    {
    boolean bool1 = false;
    int i = getState("mass", 0);
    int j = getState("happy", 0);
    int k = getState("clean", 0);
    boolean bool2 = bool1;
    if (i == 72) // Mass equals 72
    {
      bool2 = bool1;
      if (j == 30) // happy = 30
      {
        bool2 = bool1;
        if (k == 0) {  // clean = 0
          bool2 = true;
        }
      }
    }
    return bool2;

    public final boolean isHappy()
    {
    int i = getStat('f');
    int j = getStat('p');
    double d = i / j;
    boolean bool;
    if ((d >= 2.0D) && (d <= 2.5D)) {
      bool = true;
    } else {
      bool = false;
    }
    return bool;
    }

The missing piece of the puzzle was...

    public final class FlareBearActivityKt
    {
      private static final int CLEAN_PER_CLEAN = 6;
      private static final int CLEAN_PER_FEED = -1;
      private static final int CLEAN_PER_PLAY = -1;
      private static final int HAPPY_PER_CLEAN = -1;
      private static final int HAPPY_PER_FEED = 2;
      private static final int HAPPY_PER_PLAY = 4;
      private static final String IV = "pawsitive_vibes!";
      private static final int MASS_PER_CLEAN = 0;
      private static final int MASS_PER_FEED = 10;
      private static final int MASS_PER_PLAY = -2;
      private static final float POOS_PER_FEED = 0.34F;
      private static final String SALT = "NaClNaClNaCl";
      private static final String TAG = "FLARE Bear";
      private static List<ImageView> poosList = (List)new ArrayList();
    }

and the activity code for clean, play and feed are:

    feed = 'f'
    clean = 'c'
    play = 'p'

this means the constraints are that
	
	happy = 2 < (feed / play) < 2.5
	
and that 
	
	ecstatic = (mass == 72) && (happy == 30) && (clean == 0)

so the answer is:

	8 feed, 4 play, clean until clean.

